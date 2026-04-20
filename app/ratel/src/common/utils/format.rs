pub fn format_with_commas(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let formatted: String = out.chars().rev().collect();
    format!("{}{}", sign, formatted)
}

pub fn format_with_commas_str(value: &str) -> String {
    let (sign, raw) = if let Some(stripped) = value.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", value)
    };

    let mut parts = raw.split('.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next();

    let mut out = String::new();
    for (idx, ch) in int_part.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let int_formatted: String = out.chars().rev().collect();

    if let Some(frac) = frac_part {
        if frac.is_empty() {
            format!("{}{}", sign, int_formatted)
        } else {
            format!("{}{}.{}", sign, int_formatted, frac)
        }
    } else {
        format!("{}{}", sign, int_formatted)
    }
}

pub fn format_number(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{}M", n / 1_000_000)
    } else if n >= 1_000 {
        format!("{}K", n / 1_000)
    } else {
        n.to_string()
    }
}
