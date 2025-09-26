pub mod kaia_local_wallet;
pub mod kaia_transaction;
pub mod local_fee_payer;

use async_trait::async_trait;
use ethers::types::{H160, Signature};
use kaia_transaction::KaiaTransaction;

#[async_trait]
pub trait KaiaWallet {
    fn address(&self) -> H160;
    async fn sign_transaction(&self, tx: &KaiaTransaction) -> Result<Signature, crate::Error>;
}
