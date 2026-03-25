use attr_voting::VotingAuthority;

fn main() {
    let authority = VotingAuthority::setup();
    let json = authority.to_json().expect("Failed to serialize authority");
    println!("{json}");
}
