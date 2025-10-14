use bdk::prelude::*;

use std::sync::Arc;
use std::time::Duration;

use abi::Abi;
use ethers::prelude::*;
use ethers::utils::{ParseUnits, parse_units};
use tokio::time::sleep;

use crate::utils::wallets::KaiaWallet;
use crate::utils::wallets::kaia_transaction::{KaiaTransaction, TransactionType};
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct CommonContract<FeePayerWallet: KaiaWallet, UserWallet: KaiaWallet> {
    pub contract: ContractInstance<Arc<Provider<Http>>, Provider<Http>>,
    pub provider: Arc<Provider<Http>>,
    pub wallet: Option<UserWallet>,
    pub fee_payer: Option<FeePayerWallet>,
}

impl<T: KaiaWallet, W: KaiaWallet> CommonContract<T, W> {
    pub fn new(contract_address: &str, abi: &'static str, provider: Arc<Provider<Http>>) -> Self {
        let contract = Contract::new(
            contract_address.parse::<Address>().unwrap(),
            serde_json::from_str::<Abi>(abi).unwrap(),
            provider.clone(),
        );

        Self {
            contract,
            provider,
            wallet: None,
            fee_payer: None,
        }
    }

    pub fn set_wallet(&mut self, wallet: W) {
        self.wallet = Some(wallet);
    }

    pub fn set_fee_payer(&mut self, fee_payer: T) {
        self.fee_payer = Some(fee_payer);
    }

    pub async fn sign_and_send_transaction_with_feepayer(&self, input: Bytes) -> Result<String> {
        let chain_id = self.provider.get_chainid().await.map_err(|e| Error::Klaytn(e.to_string()))?;
        tracing::debug!("chain id: {}", chain_id);
        let gas = 9000000;
        let gas_price = match parse_units("750", "gwei").map_err(|e| Error::Klaytn(e.to_string()))? {
            ParseUnits::U256(x) => x,
            ParseUnits::I256(_) => return Err(Error::Klaytn("parse_units error".to_string())),
        };
        let value = U256::from(0);
        let w = match self.wallet.as_ref() {
            Some(w) => w,
            None => return Err(Error::Klaytn("wallet is None".to_string())),
        };

        let from = w.address();
        let to = self.contract.address();
        let tx_type = TransactionType::FeeDelegatedSmartContractExecution;
        let nonce = self.provider.get_transaction_count(from, None).await.map_err(|e| Error::Klaytn(e.to_string()))?;

        let tx = KaiaTransaction::new(
            tx_type,
            Some(from),
            Some(to),
            Some(U256::from(gas)),
            Some(gas_price),
            Some(value),
            Some(input.to_vec()),
            Some(nonce),
        );

        let fp = self.fee_payer.as_ref().unwrap();
        let fp_sig = fp.sign_transaction(&tx).await?;

        let sig = w.sign_transaction(&tx).await?;
        tracing::debug!("sig: {:?}", sig);

        let rlp = tx.to_tx_hash_rlp(sig, fp.address(), fp_sig);

        let rlp = format!("0x{}", hex::encode(rlp).to_string());
        tracing::debug!("rlp: {}", rlp);

        let res: std::result::Result<JsonRpcResponse<String>, Error> =
            reqwest::Client::new()
            .post(self.provider.url().as_str())
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "kaia_sendRawTransaction",
                "id": 1,
                "params": [rlp],
            }))
            .send()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()));


        let tx_hash = match res {
            Ok(res) => {
                if res.result.is_none() {
                    return Err(Error::Klaytn("sendRawTransaction error".to_string()));
                }
                res.result.unwrap()
            }
            Err(e) => {
                tracing::error!("sendRawTransaction error: {}", e);
                return Err(Error::Klaytn("sendRawTransaction error".to_string()));
            }
        };

        tracing::debug!("tx hash: {}", tx_hash);

        let mut status = "".to_string();

        for _ in 0..3 {
            sleep(Duration::from_secs(1)).await;
            let res: std::result::Result<JsonRpcResponse<TransactionReceipt>, Error> =
                reqwest::Client::new()
                    .post(self.provider.url().as_str())
                    .json(&serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": 1,
                        "method": "kaia_getTransactionReceipt",
                        "params": [tx_hash],
                    }))
                    .send()
                    .await
                    .map_err(|e| {
                        tracing::warn!("getTransactionReceipt request error: {}", e);
                        Error::Klaytn(e.to_string())
                    })?
                    .json::<JsonRpcResponse<TransactionReceipt>>()
                    .await
                    .map_err(|e| {
                        tracing::warn!("getTransactionReceipt parse error: {}", e);
                        Error::Klaytn(e.to_string())
                    });

            let res = match res {
                Ok(res) => res,
                Err(e) => {
                    tracing::warn!("getTransactionReceipt error: {}", e);
                    continue;
                }
            };

            tracing::debug!("receipt {:?}", res);

            match res.result {
                Some(v) => {
                    status = v.status;
                    break;
                }
                None => "".to_string(),
            };
        }

        if status == "0x1" {
            Ok(tx_hash)
        } else {
            Err(Error::Klaytn("internal error".to_string()))
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub block_number: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    // use crate::utils::wallets::kaia_local_wallet::KaiaLocalWallet;


    // use super::*;

    #[cfg(feature = "full-test")]
    #[tokio::test]
    async fn test_mint_batch() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(false)
            .try_init();

        let provider = Provider::<Http>::try_from("https://public-en-kairos.node.kaia.io").unwrap();
        let provider = std::sync::Arc::new(provider);

        let mut c = CommonContract::new(env!("CONTRACT_INCHEON_CONTENTS"), provider.clone());

        let w = KaiaLocalWallet::new(env!("KLAYTN_OWNER_KEY"), provider.clone())
            .await
            .unwrap();

        let f = crate::wallets::local_fee_payer::LocalFeePayer::new(
            env!("KLAYTN_FEEPAYER_ADDR"),
            env!("KLAYTN_FEEPAYER_KEY"),
            provider.clone(),
        )
        .await
        .unwrap();

        c.set_wallet(w);
        c.set_fee_payer(f);

        c.mint_batch(
            "0xb9a72033A3339B82DEf38d70c5e373a03a45fA0b".to_string(),
            [1].to_vec(),
            [1].to_vec(),
        )
        .await
        .unwrap();
    }
}
