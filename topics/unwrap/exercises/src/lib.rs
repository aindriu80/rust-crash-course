use std::str::FromStr;

pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let parsed_a = a.parse::<u32>().expect("Failed to parse variable");
    let parsed_b = b.parse::<u32>().expect("Failed to parse variable");

    parsed_a + parsed_b
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    match (x, y) {
        (Some(val_x), Some(val_y)) => val_x + val_y,
        _ => 0, // handle the case where either x or y is none.
    }
}
