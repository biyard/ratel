#[derive(Debug, Clone, Copy)]
pub struct DidConfig {
    pub bbs_bls_x: &'static str,
    pub bbs_bls_y: &'static str,
    pub bbs_bls_d: &'static str,
    pub bbs_bls_crv: &'static str,
    pub p256_x: &'static str,
    pub p256_y: &'static str,
    pub p256_d: &'static str,
    pub p256_crv: &'static str,
}

impl Default for DidConfig {
    fn default() -> Self {
        if option_env!("BBS_BLS_X").is_some() {
            Self {
                bbs_bls_x: option_env!("BBS_BLS_X").expect("You must set BBS_BLS_X"),
                bbs_bls_y: option_env!("BBS_BLS_Y").expect("You must set BBS_BLS_Y"),
                bbs_bls_d: option_env!("BBS_BLS_D").expect("You must set BBS_BLS_D"),
                bbs_bls_crv: option_env!("BBS_BLS_CRV").expect("You must set BBS_BLS_CRV"),
                p256_x: option_env!("P256_X").expect("You must set P256_X"),
                p256_y: option_env!("P256_Y").expect("You must set P256_Y"),
                p256_d: option_env!("P256_D").expect("You must set P256_D"),
                p256_crv: option_env!("P256_CRV").expect("You must set P256_CRV"),
            }
        } else {
            // FIXME: generate DIDs keys
            // let rng = &mut rand::thread_rng();
            // let keys = ssi::bbs::generate_secret_key(rng);

            Self {
                bbs_bls_x: option_env!("BBS_BLS_X").expect("You must set BBS_BLS_X"),
                bbs_bls_y: option_env!("BBS_BLS_Y").expect("You must set BBS_BLS_Y"),
                bbs_bls_d: option_env!("BBS_BLS_D").expect("You must set BBS_BLS_D"),
                bbs_bls_crv: option_env!("BBS_BLS_CRV").expect("You must set BBS_BLS_CRV"),
                p256_x: option_env!("P256_X").expect("You must set P256_X"),
                p256_y: option_env!("P256_Y").expect("You must set P256_Y"),
                p256_d: option_env!("P256_D").expect("You must set P256_D"),
                p256_crv: option_env!("P256_CRV").expect("You must set P256_CRV"),
            }
        }
    }
}
