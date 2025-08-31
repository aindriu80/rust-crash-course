pub fn first<T, U>(t: (T, U)) -> T {
    t.0
}

pub fn last<T, U>(t: (T, U)) -> U {
    t.1
}

#[derive(Debug)]
pub struct Rectangle<T: std::fmt::Debug> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}
