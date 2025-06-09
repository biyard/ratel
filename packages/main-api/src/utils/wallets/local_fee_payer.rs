use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    core::k256::ecdsa::SigningKey,
    middleware::{MiddlewareBuilder, SignerMiddleware},
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet, to_eip155_v},
    types::{H160, Signature},
};

use super::{KaiaWallet, kaia_transaction::KaiaTransaction};
use crate::Result;

#[derive(Debug, Clone)]
pub struct LocalFeePayer {
    pub wallet: LocalWallet,
    pub provider: SignerMiddleware<Arc<Provider<Http>>, Wallet<SigningKey>>,
    pub chain_id: u64,
    pub address: H160,
}

impl LocalFeePayer {
    pub async fn new(
        address: &str,
        private_key: &str,
        provider: Arc<Provider<Http>>,
    ) -> Result<Self> {
        let chain_id = provider
            .get_chainid()
            .await
            .map_err(|e| crate::Error::Klaytn(e.to_string()))?;
        let wallet = private_key
            .parse::<LocalWallet>()
            .expect("invalid fee payer private key")
            .with_chain_id(chain_id.as_u64());
        let provider = provider.with_signer(wallet.clone());
        tracing::debug!("wallet chain id: {:?}", wallet.chain_id());

        Ok(Self {
            address: address.parse().expect("invalid feepayer address"),
            wallet,
            provider,
            chain_id: chain_id.as_u64(),
        })
    }
}

#[async_trait]
impl KaiaWallet for LocalFeePayer {
    fn address(&self) -> H160 {
        self.wallet.address()
    }

    async fn sign_transaction(&self, tx: &KaiaTransaction) -> Result<Signature> {
        let hash = tx.to_sig_fee_payer_hash(self.address(), self.chain_id);
        let mut signature = self
            .wallet
            .sign_hash(hash)
            .map_err(|e| crate::Error::Klaytn(e.to_string()))?;
        signature.v = to_eip155_v(signature.v as u8 - 27, self.chain_id);

        tracing::debug!("fee payer signature: {:?}", signature);

        Ok(signature)
    }
}
