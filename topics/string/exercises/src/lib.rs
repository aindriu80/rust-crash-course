pub fn hello() -> String {
    format!("Hello Rust")
}

pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn append(mut s: String) -> String {
    s.push_str("!");
    s
}
