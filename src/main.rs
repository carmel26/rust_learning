#![allow(warnings)]

// working with UTF-8 encoded strings

fn main() {
  // 1
  let s = "Whatever data we have".to_string();
// 2
  let s : String = String::from("Whatever too, world!");
// mutate the variable [push to end of the string]
  let mut s = String::from("Foo"); 
  s.push_str(" bar");

  //for one character
  s.push('!');

  println!("the value of S is = {}", s);

  // to combine two strings, we can use the + operator
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved

  println!("the value of S3 is = {}", s3);

}