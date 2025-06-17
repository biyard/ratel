use regex::Regex;

pub fn check_test_keyword(v: Option<&str>) -> bool {
    let pattern = Regex::new(r"(?i)(?:^|\s)(test|테스트)\w*").unwrap();
    let value = v.unwrap_or("");
    pattern.is_match(value)
}
