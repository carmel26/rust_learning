fn main() {
    println!("Working with understanding reference!");  
    let x: i32 = 25;
    let r: &i32 = &x;
    println!("The value of x is: {}", x);
    println!("The value of r is: {}", r);
}