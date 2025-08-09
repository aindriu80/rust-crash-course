pub fn first(t: (bool, u32, char)) -> bool {
    let (first_element, _, _) = t;
    first_element
}

pub fn last(t: (bool, u32, char)) -> char {
    let (_, _, last_element) = t;
    last_element
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}

fn main() {
    let my_tuple = (5, 10);
    let swapped_tuple = swap(my_tuple);
    println!("Original: {:?}", my_tuple); // Output: Original: (5, 10)
    println!("Swapped: {:?}", swapped_tuple); // Output: Swapped: (10, 5)
}
