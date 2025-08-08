pub fn eq(x: char, y: char) -> bool {
    x == y
}

pub fn add(x: f32, y: f32, z: f32) -> f32 {
    x + y + z
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x as f32 + y as f32 + z
}

fn main() {
    let i: i32 = -1;
    let u: u32 = i as u32;

    let i_max = i32::MAX;
    let u_max = u32::MIN;
    println!("{i} as u32 = {u}");
    println!("i max: {i_max}");
    println!("u min: {u_max}");
}
