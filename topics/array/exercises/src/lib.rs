pub fn zeros() -> [u32; 100] {
    [0; 100]
}

pub fn first_3(s: &[u32]) -> &[u32] {
    if s.len() < 3 {
        return s;
    }
    &s[..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let len = s.len();

    if len < 3 {
        return s;
    }

    &s[len - 3..]
}
fn main() {
    let my_array = [1, 2, 3, 4, 5, 6];
    let my_slice: &[u32] = &my_array;

    let last_three = last_3(my_slice);

    println!("{:?}", last_three); // Output: [4, 5, 6]

    let short_array = [1, 2];
    let short_slice: &[u32] = &short_array;
    let last_of_short = last_3(short_slice);
    println!("{:?}", last_of_short); //Output: [1, 2]
}
