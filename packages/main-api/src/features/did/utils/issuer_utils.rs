use crate::*;
use ssi::claims::vc::{syntax::NonEmptyVec, v1::JsonCredential};
use static_iref::uri;

// Defines the shape of our custom claims.
#[derive(Serialize, Deserialize)]
pub struct MyClaims {
    name: String,
    email: String,
}

// Defines the shape of our custom claims.
#[derive(Serialize, Deserialize)]
pub struct MyCredentialSubject {
    #[serde(rename = "https://example.org/#name")]
    name: String,

    #[serde(rename = "https://example.org/#email")]
    email: String,
}

pub async fn sign_bbs(
    _message: &[u8],
) -> Result<DataIntegrity<JsonCredential<MyCredentialSubject>, AnySuite>> {
    sign(config::get().did.bbs_bls_key.into(), _message).await
}

// pub async fn sign_p256(
//     _message: &[u8],
// ) -> Result<DataIntegrity<JsonCredential<MyCredentialSubject>, AnySuite>> {
//     sign(config::get().did.p256_key.into(), _message).await;
// }

pub async fn sign(
    key: JWK,
    _message: &[u8],
) -> Result<DataIntegrity<JsonCredential<MyCredentialSubject>, AnySuite>> {
    let credential = JsonCredential::<MyCredentialSubject>::new(
        Some(uri!("https://ratel.foundation/#CredentialId").to_owned()), // id
        uri!("https://ratel.foundation/#Issuer").to_owned().into(),      // issuer
        DateTime::now().into(),                                          // issuance date
        NonEmptyVec::new(MyCredentialSubject {
            name: "John Smith".to_owned(),
            email: "john.smith@example.org".to_owned(),
        }),
    );

    let did = DIDJWK::generate_url(&key.to_public());

    let vm_resolver = DIDJWK.into_vm_resolver();
    let signer = SingleSecretSigner::new(key.clone()).into_local();
    let verification_method = did.into_iri().into();
    let cryptosuite = AnySuite::pick(&key, Some(&verification_method))
        .expect("could not find appropriate cryptosuite");
    cryptosuite
        .sign(
            credential,
            &vm_resolver,
            &signer,
            ProofOptions::from_method(verification_method),
        )
        .await
        .map_err(|e| Error::Signature(e.to_string()))
}
