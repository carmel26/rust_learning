fn main() {
    // u8, u16, u32, u64, u128, usize
    // i8, i16, i32, i64, i128, isize
    let x : i32 = -5;
    let y : u64 = 10;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y); 

    //  =======
    // Floating point types: f32, f64
    let pi: f64 = 3.14;
    println!("Value of pi: {}", pi);

    // ========
    // Boolean type: bool
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    // ========
    // Character type: char
    let letter: char = 'R';
    println!("Character: {}", letter);
}
