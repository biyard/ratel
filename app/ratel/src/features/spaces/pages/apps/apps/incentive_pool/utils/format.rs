use crate::features::spaces::pages::apps::apps::incentive_pool::models::SpaceIncentiveToken;

pub(crate) fn usdt_tokens(tokens: &[SpaceIncentiveToken]) -> Vec<SpaceIncentiveToken> {
    tokens
        .iter()
        .filter(|item| item.symbol.eq_ignore_ascii_case("USDT"))
        .cloned()
        .collect()
}

pub(crate) fn default_usdt_token_address(tokens: &[SpaceIncentiveToken]) -> Option<String> {
    usdt_tokens(tokens)
        .into_iter()
        .next()
        .map(|item| item.token_address)
}

pub(crate) fn is_valid_usdt_input(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }

    let mut dot_count = 0;
    for c in value.chars() {
        if c == '.' {
            dot_count += 1;
            if dot_count > 1 {
                return false;
            }
            continue;
        }

        if !c.is_ascii_digit() {
            return false;
        }
    }

    if value == "." {
        return true;
    }

    value.parse::<f64>().map(|v| v >= 0.0).unwrap_or(false)
}

pub(crate) fn format_token_balance(balance: &str, decimals: i64) -> String {
    if balance.is_empty() || !balance.chars().all(|c| c.is_ascii_digit()) {
        return "0".to_string();
    }

    let decimals = decimals.max(0) as usize;
    let normalized = balance.trim_start_matches('0');
    if normalized.is_empty() {
        return "0".to_string();
    }

    if decimals == 0 {
        return with_thousands_separator(normalized);
    }

    if normalized.len() <= decimals {
        let mut fractional = format!("{:0>width$}", normalized, width = decimals);
        while fractional.ends_with('0') {
            fractional.pop();
        }

        if fractional.len() > 2 {
            fractional.truncate(2);
            while fractional.ends_with('0') {
                fractional.pop();
            }
        }

        if fractional.is_empty() {
            return "0".to_string();
        }

        return format!("0.{fractional}");
    }

    let split = normalized.len() - decimals;
    let whole = &normalized[..split];
    let mut fractional = normalized[split..].to_string();

    while fractional.ends_with('0') {
        fractional.pop();
    }

    if fractional.len() > 2 {
        fractional.truncate(2);
        while fractional.ends_with('0') {
            fractional.pop();
        }
    }

    let whole = with_thousands_separator(whole);
    if fractional.is_empty() {
        whole
    } else {
        format!("{whole}.{fractional}")
    }
}

pub(crate) fn incentive_explorer_url(address: &str) -> Option<String> {
    let address = address.trim();
    if address.is_empty() {
        return None;
    }

    Some(format!("{}/address/{}", explorer_base_url(), address))
}

fn with_thousands_separator(value: &str) -> String {
    let mut result = String::new();

    for (idx, ch) in value.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    result.chars().rev().collect()
}

fn explorer_base_url() -> &'static str {
    match option_env!("ENV").unwrap_or("local") {
        "prod" | "production" => "https://kaiascan.io",
        _ => "https://kairos.kaiascan.io",
    }
}
