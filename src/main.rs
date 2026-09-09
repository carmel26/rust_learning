fn main() {
  // work with shadowing variables
//   means creating another variable from an existing variable with the same name

  let x = 12; // first variable with 12
  let x = x + 3; // second variable with 15
  println!("x = {}", x);
  {
    let x = x * 2; //third variable with 30
    println!(" the new value is = {}", x);
  }

  println!("the value of x out of the block is = {}", x); // the value of x out of the block is 15
}