use rabe::utils::policy::pest::PolicyLanguage;

pub fn vote_policy(voter_id: &str) -> (String, PolicyLanguage) {
    let policy = format!("\"ratel-authority\" or \"voter-{voter_id}\"");
    (policy, PolicyLanguage::HumanPolicy)
}
