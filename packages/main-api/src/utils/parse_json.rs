// FIXME: Change Prompt to extract only JSON part
pub fn parse_json<T>(text: &str) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    let json_str = if let (Some(start), Some(end)) = (text.find("```json"), text.rfind("```")) {
        &text[start + 7..end]
    } else if let (Some(start), Some(end)) = (text.find('{'), text.rfind('}')) {
        &text[start..=end]
    } else {
        return None;
    };

    serde_json::from_str(json_str.trim()).ok()
}
