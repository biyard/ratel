use crate::utils::evm_utils;
use crate::*;
use ethers::{
    signers::{LocalWallet, Signer},
    utils::hex::ToHexExt,
};
use x402_rs::types::EvmAddress;

#[derive(Debug, Clone, Copy)]
pub struct X402Config {
    pub evm_wallet: &'static LocalWallet,
    pub facilitator_url: &'static str,
}

impl X402Config {
    pub fn address(&self) -> EvmAddress {
        let addr = self.evm_wallet.address().encode_hex();

        addr.parse::<EvmAddress>()
            .expect("Failed to parse EVM address from wallet")
    }
}

impl Default for X402Config {
    fn default() -> Self {
        Self {
            facilitator_url: option_env!("X402_FACILITATOR_URL").unwrap_or("https://x402.org/facilitator/"),
            evm_wallet: option_env!("X402_EVM_PRIVATE_KEY")
                .map(|pk| {
                    let w = pk.parse::<LocalWallet>()
                        .expect("X402_EVM_PRIVATE_KEY is invalid");

                    Box::leak(Box::new(w))
                })
                .unwrap_or_else(|| {
                    tracing::warn!(
                        "X402_EVM_PRIVATE_KEY not set, generating a random wallet. Some features may not work properly."
                    );

                    let w = evm_utils::create_account();

                    Box::leak(Box::new(w))
                }),
        }
    }
}
