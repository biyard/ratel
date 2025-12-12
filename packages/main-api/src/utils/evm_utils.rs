use ethers::prelude::*;
use ethers::signers::LocalWallet;

pub fn create_account() -> LocalWallet {
    // Generate a random wallet
    let mut rng = ethers::core::rand::thread_rng();
    let wallet: LocalWallet = LocalWallet::new(&mut rng);

    wallet
}
