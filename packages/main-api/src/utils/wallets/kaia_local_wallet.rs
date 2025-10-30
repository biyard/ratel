use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::{MiddlewareBuilder, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet, to_eip155_v},
    types::Signature,
};

use super::{KaiaWallet, kaia_transaction::KaiaTransaction};
use crate::Result;

#[derive(Debug, Clone)]
pub struct KaiaLocalWallet {
    pub wallet: LocalWallet,
    pub provider: SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    pub chain_id: u64,
}

impl KaiaLocalWallet {
    pub async fn new(private_key: &str, provider: Arc<Provider<Http>>) -> Result<Self> {
        let chain_id = provider
            .get_chainid()
            .await
            .map_err(|e| crate::Error::Klaytn(e.to_string()))?;
        let wallet = private_key
            .parse::<LocalWallet>()
            .map_err(|e| crate::Error::Klaytn(e.to_string()))?
            .with_chain_id(chain_id.as_u64());
        let provider = provider.with_signer(wallet.clone());
        tracing::debug!("wallet chain id: {:?}", wallet.chain_id());
        Ok(Self {
            wallet,
            provider,
            chain_id: chain_id.as_u64(),
        })
    }
}

#[async_trait]
impl KaiaWallet for KaiaLocalWallet {
    fn address(&self) -> ethers::types::H160 {
        self.wallet.address()
    }

    async fn sign_transaction(&self, tx: &KaiaTransaction) -> Result<Signature> {
        let hash = tx.to_sig_hash(self.chain_id);
        let mut signature = self
            .wallet
            .sign_hash(hash)
            .map_err(|e| crate::Error::Klaytn(e.to_string()))?;

        signature.v = to_eip155_v(signature.v as u8 - 27, self.chain_id);
        tracing::debug!("signature: {:?}", signature);

        Ok(signature)
    }
}
