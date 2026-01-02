use calamine::{Data, DataType as _};

pub fn cell_string(v: &Data) -> Option<String> {
    if let Some(s) = v.get_string() {
        let t = s.trim();
        return if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        };
    }
    if let Some(f) = v.get_float() {
        if !f.is_nan() {
            return Some(format!("{}", f));
        }
    }
    if let Some(i) = v.get_int() {
        return Some(format!("{}", i));
    }
    if let Some(b) = v.get_bool() {
        return Some(format!("{}", b));
    }
    None
}
