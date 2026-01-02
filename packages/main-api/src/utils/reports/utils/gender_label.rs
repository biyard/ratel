use calamine::{Data, DataType as _};

pub fn gender_label(v: &Data) -> Option<&'static str> {
    let n = if let Some(i) = v.get_int() {
        i
    } else if let Some(f) = v.get_float() {
        f as i64
    } else if let Some(s) = v.get_string() {
        s.trim().parse::<i64>().ok()?
    } else {
        return None;
    };

    match n {
        1 => Some("남성"),
        2 => Some("여성"),
        _ => None,
    }
}
