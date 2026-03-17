use crate::features::posts::*;

pub fn convert_number_to_string(n: i64) -> String {
    let suffixes = ["K", "M", "B"];
    let mut value = n as f64;
    let mut i = 0;

    while value >= 1000.0 && i < suffixes.len() {
        value /= 1000.0;
        i += 1;
    }

    if i == 0 {
        format!("{}", n)
    } else {
        format!("{} {}", value as i64, suffixes[i - 1])
    }
}
