use base64::{engine::general_purpose, Engine};
use ring::signature::{UnparsedPublicKey, ED25519};
use simple_asn1::{
    oid, to_der,
    ASN1Block::{BitString, ObjectIdentifier, Sequence},
};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SignatureAlgorithm {
    EdDSA,
}

impl Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "eddsa")
    }
}

impl FromStr for SignatureAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eddsa" => Ok(SignatureAlgorithm::EdDSA),
            _ => Err(format!("Unknown signature algorithm: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Signature {
    pub signature: Vec<u8>,
    pub algorithm: SignatureAlgorithm,
    pub public_key: Vec<u8>,
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sig = general_purpose::STANDARD.encode(&self.signature);
        let public_key = general_purpose::STANDARD.encode(&self.public_key);

        write!(f, "{}:{}:{}", self.algorithm, public_key, sig)
    }
}

impl FromStr for Signature {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid signature format: {}", s));
        }
        tracing::debug!("Parsing signature: {:?}", parts);

        let algorithm = SignatureAlgorithm::from_str(parts[0])?;
        let public_key = general_purpose::STANDARD
            .decode(parts[1])
            .map_err(|e| format!("failed to decode public key: {e}"))?;
        let signature = general_purpose::STANDARD
            .decode(parts[2])
            .map_err(|e| format!("failed to decode signature: {e}"))?;
        let sig = Signature {
            signature,
            algorithm,
            public_key,
        };

        Ok(sig)
    }
}

impl Signature {
    pub fn verify(&self, msg: &str) -> Result<String, String> {
        match self.algorithm {
            SignatureAlgorithm::EdDSA => {
                tracing::debug!("Verifying signature: EdDSA {} {:?}", msg, self);
                let pk = self.public_key.clone();
                let public_key = UnparsedPublicKey::new(&ED25519, &pk);
                public_key
                    .verify(msg.as_bytes(), &self.signature)
                    .map_err(|e| format!("Verification failed: {}", e))?;

                self.principal()
            }
        }
    }

    pub fn principal(&self) -> Result<String, String> {
        let public_key = self.public_key.clone();

        let id_ed25519 = oid!(1, 3, 101, 112);
        let algorithm = Sequence(0, vec![ObjectIdentifier(0, id_ed25519)]);
        let subject_public_key = BitString(0, public_key.len() * 8, public_key);
        let subject_public_key_info = Sequence(0, vec![algorithm, subject_public_key]);
        let der_public_key = to_der(&subject_public_key_info).unwrap();
        let wallet_address = candid::Principal::self_authenticating(der_public_key);
        Ok(wallet_address.to_text())
    }
}
