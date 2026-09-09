#![allow(warnings)]
fn main() {
  // control flow
  let age: u32 = 18;
  let is_adult: bool = if age >= 18 {true} else {false}; 

    if is_adult{
        println!("You are an adult.");
    }else{
        println!("You are not an adult.");
    }

    // initialize a variable with control flow
    let number = if is_adult{23} else {9};
    println!("The number is: {}", number);
}