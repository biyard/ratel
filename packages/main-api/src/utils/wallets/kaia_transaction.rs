use bdk::prelude::*;
use ethers::prelude::*;
use rlp::RlpStream;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    #[default]
    Legacy,
    ValueTransfer,
    FeeDelegatedValueTransfer,
    ValueTransferMemo,
    FeeDelegatedValueTransferMemo,
    SmartContractDeploy,
    FeeDelegatedSmartContractDeploy,
    SmartContractExecution,
    FeeDelegatedSmartContractExecution,
    AccountUpdate,
    FeeDelegatedAccountUpdate,
    Cancel,
    FeeDelegatedCancel,
}

impl TransactionType {
    pub fn to_tx_type_string(&self) -> String {
        match self {
            TransactionType::Legacy => "LEGACY".to_string(),
            TransactionType::ValueTransfer => "VALUE_TRANSFER".to_string(),
            TransactionType::FeeDelegatedValueTransfer => {
                "FEE_DELEGATED_VALUE_TRANSFER".to_string()
            }
            TransactionType::ValueTransferMemo => "VALUE_TRANSFER_MEMO".to_string(),
            TransactionType::FeeDelegatedValueTransferMemo => {
                "FEE_DELEGATED_VALUE_TRANSFER_MEMO".to_string()
            }
            TransactionType::SmartContractDeploy => "SMART_CONTRACT_DEPLOY".to_string(),
            TransactionType::FeeDelegatedSmartContractDeploy => {
                "FEE_DELEGATED_SMART_CONTRACT_DEPLOY".to_string()
            }
            TransactionType::SmartContractExecution => "SMART_CONTRACT_EXECUTION".to_string(),
            TransactionType::FeeDelegatedSmartContractExecution => {
                "FEE_DELEGATED_SMART_CONTRACT_EXECUTION".to_string()
            }
            TransactionType::AccountUpdate => "ACCOUNT_UPDATE".to_string(),
            TransactionType::FeeDelegatedAccountUpdate => {
                "FEE_DELEGATED_ACCOUNT_UPDATE".to_string()
            }
            TransactionType::Cancel => "CANCEL".to_string(),
            TransactionType::FeeDelegatedCancel => "FEE_DELEGATED_CANCEL".to_string(),
        }
    }

    pub fn to_tx_type_code(&self) -> u8 {
        match self {
            TransactionType::Legacy => 0x0,
            TransactionType::ValueTransfer => 0x8,
            TransactionType::FeeDelegatedValueTransfer => 0x9,
            TransactionType::ValueTransferMemo => 0x10,
            TransactionType::FeeDelegatedValueTransferMemo => 0x11,
            TransactionType::AccountUpdate => 0x20,
            TransactionType::FeeDelegatedAccountUpdate => 0x21,
            TransactionType::SmartContractDeploy => 0x28,
            TransactionType::FeeDelegatedSmartContractDeploy => 0x29,
            TransactionType::SmartContractExecution => 0x30,
            TransactionType::FeeDelegatedSmartContractExecution => 0x31,
            TransactionType::Cancel => 0x38,
            TransactionType::FeeDelegatedCancel => 0x39,
        }
    }
}

impl TryFrom<&str> for TransactionType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "VALUE_TRANSFER" => Ok(TransactionType::ValueTransfer),
            "FEE_DELEGATED_VALUE_TRANSFER" => Ok(TransactionType::FeeDelegatedValueTransfer),
            "VALUE_TRANSFER_MEMO" => Ok(TransactionType::ValueTransferMemo),
            "FEE_DELEGATED_VALUE_TRANSFER_MEMO" => {
                Ok(TransactionType::FeeDelegatedValueTransferMemo)
            }
            "SMART_CONTRACT_DEPLOY" => Ok(TransactionType::SmartContractDeploy),
            "FEE_DELEGATED_SMART_CONTRACT_DEPLOY" => {
                Ok(TransactionType::FeeDelegatedSmartContractDeploy)
            }
            "SMART_CONTRACT_EXECUTION" => Ok(TransactionType::SmartContractExecution),
            "FEE_DELEGATED_SMART_CONTRACT_EXECUTION" => {
                Ok(TransactionType::FeeDelegatedSmartContractExecution)
            }
            "ACCOUNT_UPDATE" => Ok(TransactionType::AccountUpdate),
            "FEE_DELEGATED_ACCOUNT_UPDATE" => Ok(TransactionType::FeeDelegatedAccountUpdate),
            "CANCEL" => Ok(TransactionType::Cancel),
            "FEE_DELEGATED_CANCEL" => Ok(TransactionType::FeeDelegatedCancel),
            "LEGACY" => Ok(TransactionType::Legacy),
            _ => Err(format!("Unknown transaction type: {}", value)),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
pub struct KaiaTransaction {
    pub tx_type: TransactionType,
    pub nonce: Option<U256>,
    pub gas_price: Option<U256>,
    pub gas: Option<U256>,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Option<U256>,
    pub input: Option<Vec<u8>>,
}

impl schemars::JsonSchema for KaiaTransaction {
    fn schema_name() -> String {
        "KlaytnTransaction".to_string()
    }

    fn json_schema(_gen: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::SchemaObject;

        let mut schema = SchemaObject::default();
        schema.metadata = Some(Box::new(schemars::schema::Metadata {
            title: Some("KlaytnTransaction".to_string()),
            description: Some("Refer to the detailed specification".to_string()),
            ..Default::default()
        }));

        schemars::schema::Schema::Object(schema)
    }
}

impl by_axum::aide::OperationInput for KaiaTransaction {}

impl by_axum::aide::OperationOutput for KaiaTransaction {
    type Inner = Self;
}

impl KaiaTransaction {
    pub fn new(
        tx_type: TransactionType,
        from: Option<Address>,
        to: Option<Address>,
        gas: Option<U256>,
        gas_price: Option<U256>,
        value: Option<U256>,
        input: Option<Vec<u8>>,
        nonce: Option<U256>,
    ) -> Self {
        KaiaTransaction {
            tx_type,
            nonce,
            gas_price,
            gas,
            from,
            to,
            value,
            input,
        }
    }

    pub fn value_with_default_zero(&self) -> U256 {
        self.value.unwrap_or(U256::from(0))
    }

    pub fn to_tx_hash_rlp(
        &self,
        signature: Signature,
        fee_payer: H160,
        fee_payer_signature: Signature,
    ) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        match self.tx_type {
            TransactionType::FeeDelegatedSmartContractExecution => {
                rlp.append(&self.tx_type.to_tx_type_code());

                rlp.begin_list(10);

                rlp_opt(&mut rlp, &self.nonce);
                rlp_opt(&mut rlp, &self.gas_price);
                rlp_opt(&mut rlp, &self.gas);
                rlp_opt(&mut rlp, &self.to);
                rlp_opt(&mut rlp, &self.value);
                rlp_opt(&mut rlp, &self.from);
                rlp_opt(&mut rlp, &self.input);

                rlp_sig(&mut rlp, &signature);

                rlp.append(&fee_payer);

                rlp_sig(&mut rlp, &fee_payer_signature);
            }
            _ => unimplemented!("unsupported type"),
        }
        rlp.out().to_vec()
    }

    // https://archive-docs.klaytn.foundation/content/klaytn/design/transactions/fee-delegation#rlp-encoding-for-signature-of-the-fee-payer-3
    pub fn to_sig_fee_payer_rlp(&self, fee_payer: H160, chain_id: u64) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        match self.tx_type {
            TransactionType::FeeDelegatedSmartContractExecution => {
                rlp.begin_list(5);
                let mut inner = RlpStream::new_list(8);

                inner.append(&self.tx_type.to_tx_type_code());
                rlp_opt(&mut inner, &self.nonce);
                rlp_opt(&mut inner, &self.gas_price);
                rlp_opt(&mut inner, &self.gas);
                rlp_opt(&mut inner, &self.to);
                rlp_opt(&mut inner, &self.value);
                rlp_opt(&mut inner, &self.from);
                rlp_opt(&mut inner, &self.input);

                rlp.append(&inner.out());

                rlp.append(&fee_payer);
                rlp.append(&chain_id);
                rlp.append(&0u8);
                rlp.append(&0u8);
            }
            _ => unimplemented!("unsupported type"),
        }
        rlp.out().to_vec()
    }

    // https://archive-docs.klaytn.foundation/content/klaytn/design/transactions/fee-delegation#rlp-encoding-for-signature-of-the-sender-3
    pub fn to_sig_rlp(&self, chain_id: u64) -> Vec<u8> {
        let mut rlp = RlpStream::new();
        match self.tx_type {
            TransactionType::FeeDelegatedSmartContractExecution => {
                rlp.begin_unbounded_list();
                let mut inner = RlpStream::new_list(8);
                inner.append(&self.tx_type.to_tx_type_code());
                rlp_opt(&mut inner, &self.nonce);
                rlp_opt(&mut inner, &self.gas_price);
                rlp_opt(&mut inner, &self.gas);
                rlp_opt(&mut inner, &self.to);
                rlp_opt(&mut inner, &self.value);
                rlp_opt(&mut inner, &self.from);
                rlp_opt(&mut inner, &self.input);

                rlp.append(&inner.out());

                rlp.append(&chain_id);
                rlp.append(&0u8);
                rlp.append(&0u8);
                rlp.finalize_unbounded_list();
            }
            _ => unimplemented!("unsupported type"),
        }
        rlp.out().to_vec()
    }

    pub fn to_sig_hash(&self, chain_id: u64) -> H256 {
        let sig_rlp = self.to_sig_rlp(chain_id);
        let hash = ethers::utils::keccak256(&sig_rlp);
        H256::from_slice(&hash)
    }

    pub fn to_sig_fee_payer_hash(&self, fee_payer: H160, chain_id: u64) -> H256 {
        let sig_rlp = self.to_sig_fee_payer_rlp(fee_payer, chain_id);
        let hash = ethers::utils::keccak256(&sig_rlp);
        H256::from_slice(&hash)
    }
}

pub fn rlp_opt<T: rlp::Encodable>(rlp: &mut rlp::RlpStream, opt: &Option<T>) {
    if let Some(inner) = opt {
        rlp.append(inner);
    } else {
        rlp.append(&"");
    }
}

pub fn rlp_sig(rlp: &mut rlp::RlpStream, sig: &Signature) {
    // let mut sig_rlp = RlpStream::new_list(3);
    rlp.begin_list(1);
    rlp.begin_list(3);
    rlp.append(&sig.v);
    rlp.append(&sig.r);
    rlp.append(&sig.s);
    // let sig_rlp = sig_rlp.out();

    // let mut sig = RlpStream::new_list(1);
    // sig.append(&sig_rlp);

    // rlp.append(&sig.out());
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://archive-docs.klaytn.foundation/content/klaytn/design/transactions/fee-delegation#rlp-encoding-example-3
    #[test]
    fn test_rlp_fd_contract_execution() {
        let tx = KaiaTransaction::new(
            TransactionType::FeeDelegatedSmartContractExecution,
            Some(Address::from_slice(
                &hex::decode("a94f5374Fce5edBC8E2a8697C15331677e6EbF0B").unwrap(),
            )),
            Some(Address::from_slice(
                &hex::decode("7b65B75d204aBed71587c9E519a89277766EE1d0").unwrap(),
            )),
            Some(U256::from(0xf4240)),
            Some(U256::from(0x19)),
            Some(U256::from(0xa)),
            Some(
                hex::decode(
                    "6353586b000000000000000000000000bc5951f055a85f41a3b62fd6f68ab7de76d299b2",
                )
                .unwrap(),
            ),
            Some(U256::from(0x4d2)),
        );

        assert_eq!(
            hex::encode(tx.to_sig_rlp(1)).as_str(),
            "f860b85bf859318204d219830f4240947b65b75d204abed71587c9e519a89277766ee1d00a94a94f5374fce5edbc8e2a8697c15331677e6ebf0ba46353586b000000000000000000000000bc5951f055a85f41a3b62fd6f68ab7de76d299b2018080"
        );

        assert_eq!(
            hex::encode(tx.to_sig_hash(1)).as_str(),
            "a5dd93af9f96fa316f0ddd84f10acb2e6eb41baaec3b42f9068c38aa1618f7e1"
        );

        let fee_payer =
            H160::from_slice(&hex::decode("5A0043070275d9f6054307Ee7348bD660849D90f").unwrap());

        assert_eq!(
            hex::encode(tx.to_sig_fee_payer_rlp(fee_payer.clone(), 1)).as_str(),
            "f875b85bf859318204d219830f4240947b65b75d204abed71587c9e519a89277766ee1d00a94a94f5374fce5edbc8e2a8697c15331677e6ebf0ba46353586b000000000000000000000000bc5951f055a85f41a3b62fd6f68ab7de76d299b2945a0043070275d9f6054307ee7348bd660849d90f018080"
        );

        assert_eq!(
            hex::encode(tx.to_sig_fee_payer_hash(fee_payer.clone(), 1)).as_str(),
            "f547d9d0041912e0daa2db2b65170a9e833877cd8482f405a11b03429fcbd554"
        );

        let sig = Signature {
            v: 0x25,
            r: U256::from("0x253aea7d2c37160da45e84afbb45f6b3341cf1e8fc2df4ecc78f14adb512dc4f"),
            s: U256::from("0x22465b74015c2a8f8501186bb5e200e6ce44be52e9374615a7e7e21c41bc27b5"),
        };

        let fee_payer_sig = Signature {
            v: 0x26,
            r: U256::from("0xe7c51db7b922c6fa2a941c9687884c593b1b13076bdf0c473538d826bf7b9d1a"),
            s: U256::from("0x5b0de2aabb84b66db8bf52d62f3d3b71b592e3748455630f1504c20073624d80"),
        };

        let rlp = tx.to_tx_hash_rlp(sig, fee_payer, fee_payer_sig);
        assert_eq!(
            hex::encode(rlp).as_str(),
            "31f8fb8204d219830f4240947b65b75d204abed71587c9e519a89277766ee1d00a94a94f5374fce5edbc8e2a8697c15331677e6ebf0ba46353586b000000000000000000000000bc5951f055a85f41a3b62fd6f68ab7de76d299b2f845f84325a0253aea7d2c37160da45e84afbb45f6b3341cf1e8fc2df4ecc78f14adb512dc4fa022465b74015c2a8f8501186bb5e200e6ce44be52e9374615a7e7e21c41bc27b5945a0043070275d9f6054307ee7348bd660849d90ff845f84326a0e7c51db7b922c6fa2a941c9687884c593b1b13076bdf0c473538d826bf7b9d1aa05b0de2aabb84b66db8bf52d62f3d3b71b592e3748455630f1504c20073624d80"
        );
    }
}
