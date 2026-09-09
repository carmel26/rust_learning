#![allow(warnings)]
fn main() {
  // loops
//   normals loops
  let mut counter: i32 = 0;
  let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    };
  };
  println!("The result is: {}", result);
  // loops labels to disambiguate Between multiple loops
  'counting_up: loop {
    println!("count = {}", counter);
    let mut remaining  = 10;
    loop {
        println!("remaining = {}", remaining);
        if remaining == 9 {
            break;
        }
        if counter == 2 {
            break 'counting_up;
        }
        remaining -= 1;
        counter -= 1;
    }
  }

  // while loops
  let mut number = 4;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  print!("HEYYYY!!!");

  // for loops
    let a = [10, 20, 30, 40, 50, 32,23];
    for element in a {
        println!("the value is: {element}");
    }

    let text = "Mwiriwe neza ga basha?";
    for word in text.split_whitespace() {
        println!("{word}");
    }
}