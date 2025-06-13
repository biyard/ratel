use ethers::prelude::*;
use std::sync::Arc;

use crate::{Error, Result, utils::wallets::KaiaWallet};

use super::common_contract::CommonContract;

#[derive(Debug, Clone)]
pub struct Erc1155Contract<FeePayerWallet: KaiaWallet, UserWallet: KaiaWallet> {
    pub contract: CommonContract<FeePayerWallet, UserWallet>,
}

impl<T: KaiaWallet, W: KaiaWallet> Erc1155Contract<T, W> {
    pub fn new(contract_address: &str, provider: Arc<Provider<Http>>) -> Self {
        let contract =
            CommonContract::new(contract_address, include_str!("./erc1155.json"), provider);

        Self { contract }
    }

    pub async fn balance_of_batch(&self, addrs: Vec<String>, ids: Vec<u64>) -> Result<Vec<U256>> {
        let addrs: Vec<Address> = addrs
            .into_iter()
            .map(|e| e.parse::<Address>().unwrap_or(Address::zero()))
            .collect();

        let ids: Vec<U256> = ids.into_iter().map(|e| U256::from(e)).collect();

        let nfts = self
            .contract
            .contract
            .method("balanceOfBatch", (addrs, ids))
            .map_err(|e| Error::Klaytn(e.to_string()))?
            .call()
            .await
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        Ok(nfts)
    }

    pub async fn mint_batch(
        &self,
        addr: String,
        ids: Vec<u64>,
        values: Vec<u64>,
    ) -> Result<String> {
        let addr = addr
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let ids: Vec<U256> = ids.into_iter().map(|e| U256::from(e)).collect();
        let values: Vec<U256> = values.into_iter().map(|e| U256::from(e)).collect();

        let input = self
            .contract
            .contract
            .method::<(H160, Vec<U256>, Vec<U256>), ()>("mintBatch", (addr, ids, values))?
            .calldata()
            .ok_or(Error::Klaytn("calldata error".to_string()))?;

        let tx_hash = self
            .contract
            .sign_and_send_transaction_with_feepayer(input)
            .await?;

        Ok(tx_hash)
    }

    pub async fn mint(&self, addr: String, id: u64) -> Result<String> {
        let addr = addr
            .parse::<Address>()
            .map_err(|e| Error::Klaytn(e.to_string()))?;

        let ids: U256 = U256::from(id);
        let values = U256::from(1);

        let input = self
            .contract
            .contract
            .method::<_, ()>("mint", (addr, ids, values))?
            .calldata()
            .ok_or(Error::Klaytn("calldata error".to_string()))?;

        let tx_hash = self
            .contract
            .sign_and_send_transaction_with_feepayer(input)
            .await?;

        Ok(tx_hash)
    }

    pub fn set_wallet(&mut self, wallet: W) {
        self.contract.set_wallet(wallet);
    }

    pub fn set_fee_payer(&mut self, fee_payer: T) {
        self.contract.set_fee_payer(fee_payer);
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

        let mut c = Erc1155Contract::new(env!("CONTRACT_INCHEON_CONTENTS"), provider.clone());

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
