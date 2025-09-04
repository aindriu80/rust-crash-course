use std::cmp::PartialOrd;

pub fn min<T>(x: T, y: T) -> T
where
    T: PartialOrd + Copy,
{
    if x < y { x } else { y }
}

pub fn zip<A, B>(a: Vec<A>, b: Vec<B>) -> Vec<(A, B)>
where
    A: Copy,
    B: Copy,
{
    let len = min::<usize>(a.len(), b.len());

    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        v.push((a[i], b[i]));
    }

    v
}
