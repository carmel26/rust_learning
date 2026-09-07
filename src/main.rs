fn main() {
    println!("Working with understanding reference!");   
    let mut _x: i32 = 25;
    let _r: &mut i32 = &mut _x;

    *_r += 1; 
    *_r -= 3;
    
    println!("The value of x is: {}", _x); 
    // println!("The value of r is: {}", _r);
}