pub trait Iterator<T> {
    fn next(&mut self) -> Option<&T>;
}

pub struct TupleIter<T> {
    pub tuple: (T, T, T),
    pub next: usize,
}

pub struct VecIter<T> {
    pub vec: Vec<T>,
    pub next: usize,
}

impl<T> Iterator<T> for TupleIter<T> {
    fn next(&mut self) -> Option<&T> {
        let result = match self.next {
            0 => Some(&self.tuple.0),
            1 => Some(&self.tuple.1),
            2 => Some(&self.tuple.2),
            _ => None,
        };

        // Advance the cursor - it is not an issue to go past 3
        self.next += 1;
        result
    }
}

impl<T> Iterator<T> for VecIter<T> {
    fn next(&mut self) -> Option<&T> {
        let result = if self.next < self.vec.len() {
            Some(&self.vec[self.next])
        } else {
            None
        };
        self.next += 1;
        result
    }
}
