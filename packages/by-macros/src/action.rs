use indexmap::IndexMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ActionRequestField {
    pub name: String,
    pub r#type: String,
}

impl ActionRequestField {
    pub fn ty(&self) -> proc_macro2::TokenStream {
        self.r#type
            .parse()
            .expect(format!("invalid type of {}: {}", self.name, self.r#type).as_str())
    }
}

/// The Actions struct holds three groups of action requests:
/// - `actions`: for the "action" key
/// - `read_actions`: for the "read_action" key
/// - `action_by_id`: for the "action_by_id" key
#[derive(Debug, PartialEq)]
pub struct Actions {
    pub actions: IndexMap<String, Vec<ActionRequestField>>,
    pub queryable: Vec<ActionRequestField>,
    pub read_actions: IndexMap<String, Vec<ActionRequestField>>,
    pub action_by_id: IndexMap<String, Vec<ActionRequestField>>,
    pub responses: IndexMap<String, Vec<ActionRequestField>>,
}

impl FromStr for Actions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse the full annotation string into top-level key=value pairs.
        let pairs = parse_key_value_pairs(s)?;

        // Prepare maps for each category.
        let mut actions: IndexMap<String, Vec<ActionRequestField>> = IndexMap::new();
        let mut read_actions: IndexMap<String, Vec<ActionRequestField>> = IndexMap::new();
        let mut action_by_id: IndexMap<String, Vec<ActionRequestField>> = IndexMap::new();
        let mut responses: IndexMap<String, Vec<ActionRequestField>> = IndexMap::new();
        let mut queryable: Vec<ActionRequestField> = Vec::new();

        for (key, value) in pairs {
            let key_lower = key.trim().to_lowercase();
            if key_lower == "action" {
                actions = parse_action_value(&value)?;
            } else if key_lower == "read_action" {
                read_actions = parse_action_value(&value)?;
            } else if key_lower == "action_by_id" {
                action_by_id = parse_action_value(&value)?;
            } else if key_lower == "response" {
                responses = parse_action_value(&value)?;
            } else if key_lower == "queryable" {
                for (_, v) in parse_action_value(&value)? {
                    queryable.extend(v);
                }
            }
        }

        Ok(Actions {
            actions,
            queryable,
            read_actions,
            action_by_id,
            responses,
        })
    }
}

/// Splits the top-level annotation string into key=value pairs.
/// It ignores commas inside quotes, parentheses, or brackets.
fn parse_key_value_pairs(s: &str) -> Result<Vec<(String, String)>, String> {
    let mut pairs = Vec::new();
    let mut token = String::new();
    let mut paren_depth = 0;
    let mut bracket_depth = 0;
    let mut in_quote = false;

    for c in s.chars() {
        match c {
            '"' => {
                in_quote = !in_quote;
                token.push(c);
            }
            '(' if !in_quote => {
                paren_depth += 1;
                token.push(c);
            }
            ')' if !in_quote => {
                if paren_depth == 0 {
                    return Err("Unmatched ')'".to_string());
                }
                paren_depth -= 1;
                token.push(c);
            }
            '[' if !in_quote => {
                bracket_depth += 1;
                token.push(c);
            }
            ']' if !in_quote => {
                if bracket_depth == 0 {
                    return Err("Unmatched ']'".to_string());
                }
                bracket_depth -= 1;
                token.push(c);
            }
            ',' if !in_quote && paren_depth == 0 && bracket_depth == 0 => {
                if !token.trim().is_empty() {
                    let (k, v) = split_key_value(&token)?;
                    pairs.push((k, v));
                }
                token.clear();
            }
            other => token.push(other),
        }
    }
    if !token.trim().is_empty() {
        let (k, v) = split_key_value(&token)?;
        pairs.push((k, v));
    }
    Ok(pairs)
}

/// Splits a token of the form "key = value" into its key and value components.
fn split_key_value(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid key=value pair: '{}'", s));
    }
    Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

/// Parses the value for an action key into an IndexMap mapping action names to their fields.
/// The value may be a single entry (e.g. "read_data") or a list (e.g. "[signup(code = String, param=u64), login]").
fn parse_action_value(s: &str) -> Result<IndexMap<String, Vec<ActionRequestField>>, String> {
    let s = s.trim();
    // Remove surrounding square brackets if present.
    let s = if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        s
    };

    // Split the value into individual action tokens using top-level commas.
    let mut actions = Vec::new();
    let mut token = String::new();
    let mut paren_depth = 0;
    // let mut angle_depth = 0;
    for c in s.chars() {
        match c {
            '(' => {
                paren_depth += 1;
                token.push(c);
            }
            ')' => {
                if paren_depth == 0 {
                    return Err("Unmatched ')' in action value".to_string());
                }
                paren_depth -= 1;
                token.push(c);
            }
            // '<' => {
            //     angle_depth += 1;
            //     token.push(c);
            // }
            // '>' => {
            //     if angle_depth == 0 {
            //         return Err("Unmatched '>' in action value".to_string());
            //     }
            //     angle_depth -= 1;
            //     token.push(c);
            // }
            ',' if paren_depth == 0 => {
                if !token.trim().is_empty() {
                    actions.push(token.trim().to_string());
                }
                token.clear();
            }
            other => token.push(other),
        }
    }
    if !token.trim().is_empty() {
        actions.push(token.trim().to_string());
    }

    // Build the IndexMap: for each action token, extract the action name and its fields.
    let mut map = IndexMap::new();
    for action in actions {
        if let Some(paren_index) = action.find('(') {
            let action_name = action[..paren_index].trim().to_string();
            let paren_end = action
                .rfind(')')
                .ok_or_else(|| format!("Missing closing parenthesis in '{}'", action))?;
            let field_str = &action[paren_index + 1..paren_end];
            let fields = parse_fields(field_str)?;
            map.insert(action_name, fields);
        } else {
            let action_name = action.trim().to_string();
            map.insert(action_name, Vec::new());
        }
    }
    Ok(map)
}

/// Parses a string of comma-separated fields (e.g. "code = String, param=u64")
/// into a vector of ActionRequestField.
fn parse_fields(s: &str) -> Result<Vec<ActionRequestField>, String> {
    let mut fields = Vec::new();
    let mut token = String::new();
    // let mut angle_depth = 0;
    for c in s.chars() {
        match c {
            // '<' => {
            //     angle_depth += 1;
            //     token.push(c);
            // }
            // '>' => {
            //     if angle_depth == 0 {
            //         return Err("Unmatched '>' in field definition".to_string());
            //     }
            //     angle_depth -= 1;
            //     token.push(c);
            // }
            ',' => {
                if !token.trim().is_empty() {
                    let field = parse_field(&token)?;
                    fields.push(field);
                }
                token.clear();
            }
            other => token.push(other),
        }
    }
    if !token.trim().is_empty() {
        let field = parse_field(&token)?;
        fields.push(field);
    }
    Ok(fields)
}

/// Parses a single field of the form "name = Type" into an ActionRequestField.
fn parse_field(s: &str) -> Result<ActionRequestField, String> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() == 1 {
        Ok(ActionRequestField {
            name: parts[0].trim().to_string(),
            r#type: "".to_string(),
        })
    } else if parts.len() == 2 {
        Ok(ActionRequestField {
            name: parts[0].trim().to_string(),
            r#type: parts[1].trim().to_string(),
        })
    } else {
        return Err(format!("Invalid field definition: '{}'", s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_parse_field() {
        let s = "code = String";
        let field = parse_field(s).unwrap();
        assert_eq!(
            field,
            ActionRequestField {
                name: "code".to_string(),
                r#type: "String".to_string()
            }
        );
    }

    #[test]
    fn test_parse_fields() {
        let s = "code = String, param = u64";
        let fields = parse_fields(s).unwrap();
        let expected = vec![
            ActionRequestField {
                name: "code".to_string(),
                r#type: "String".to_string(),
            },
            ActionRequestField {
                name: "param".to_string(),
                r#type: "u64".to_string(),
            },
        ];
        assert_eq!(fields, expected);
    }

    #[test]
    fn test_parse_action_value_single_no_params() {
        // e.g. "login"
        let input = "login";
        let map = parse_action_value(input).unwrap();
        let mut expected = IndexMap::new();
        expected.insert("login".to_string(), Vec::new());
        assert_eq!(map, expected);
    }

    #[test]
    fn test_parse_action_value_single_with_params() {
        // e.g. "signup(code = String, param = u64)"
        let input = "signup(code = String, param = u64)";
        let map = parse_action_value(input).unwrap();
        let mut expected = IndexMap::new();
        expected.insert(
            "signup".to_string(),
            vec![
                ActionRequestField {
                    name: "code".to_string(),
                    r#type: "String".to_string(),
                },
                ActionRequestField {
                    name: "param".to_string(),
                    r#type: "u64".to_string(),
                },
            ],
        );
        assert_eq!(map, expected);
    }

    #[test]
    fn test_parse_action_value_list() {
        // e.g. "[signup(code = String, param = u64), login]"
        let input = "[signup(code = String, param = u64), login]";
        let map = parse_action_value(input).unwrap();
        let mut expected = IndexMap::new();
        expected.insert(
            "signup".to_string(),
            vec![
                ActionRequestField {
                    name: "code".to_string(),
                    r#type: "String".to_string(),
                },
                ActionRequestField {
                    name: "param".to_string(),
                    r#type: "u64".to_string(),
                },
            ],
        );
        expected.insert("login".to_string(), Vec::new());
        assert_eq!(map, expected);
    }

    #[test]
    fn test_parse_query_value_list() {
        let input = "[(code = String, param = u64)]";
        let map = parse_action_value(input).unwrap();
        let mut expected = IndexMap::new();
        expected.insert(
            "".to_string(),
            vec![
                ActionRequestField {
                    name: "code".to_string(),
                    r#type: "String".to_string(),
                },
                ActionRequestField {
                    name: "param".to_string(),
                    r#type: "u64".to_string(),
                },
            ],
        );
        assert_eq!(map, expected);
    }

    #[test]
    fn test_actions_from_str_full() {
        let input = r#"
            base = "/auth/v1",
            queryable = [(sort = Sorter)],
            action = [signup(code = String, param = u64), login(test = Vec<String>)],
            read_action = read_data,
            action_by_id = [get_data, update_data],
            response = [signup(UserResponse)]
        "#;
        let actions = input.parse::<Actions>().unwrap();

        let mut expected_actions = IndexMap::new();
        expected_actions.insert(
            "signup".to_string(),
            vec![
                ActionRequestField {
                    name: "code".to_string(),
                    r#type: "String".to_string(),
                },
                ActionRequestField {
                    name: "param".to_string(),
                    r#type: "u64".to_string(),
                },
            ],
        );
        expected_actions.insert(
            "login".to_string(),
            vec![ActionRequestField {
                name: "test".to_string(),
                r#type: "Vec<String>".to_string(),
            }],
        );

        let mut expected_read_actions = IndexMap::new();
        expected_read_actions.insert("read_data".to_string(), Vec::new());

        let mut expected_action_by_id = IndexMap::new();
        expected_action_by_id.insert("get_data".to_string(), Vec::new());
        expected_action_by_id.insert("update_data".to_string(), Vec::new());

        let mut expected_responses = IndexMap::new();
        expected_responses.insert(
            "signup".to_string(),
            vec![ActionRequestField {
                name: "UserResponse".to_string(),
                r#type: "".to_string(),
            }],
        );

        let expected = Actions {
            actions: expected_actions,
            queryable: vec![ActionRequestField {
                name: "sort".to_string(),
                r#type: "Sorter".to_string(),
            }],
            read_actions: expected_read_actions,
            action_by_id: expected_action_by_id,
            responses: expected_responses,
        };

        assert_eq!(actions, expected);
    }
}
