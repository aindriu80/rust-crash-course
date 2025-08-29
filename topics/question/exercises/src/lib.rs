fn parse(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err("Failed to parse string into u32".to_string()),
    }
}

pub fn sum(nums: &[&str]) -> Result<u32, String> {
    nums.iter()
        .map(|s| parse(*s)) // call the helper for each &str
        .try_fold(0u32, |acc, val| val.map(|x| acc + x))
}
