#![allow(warnings)]

// let's work with Option T
// enum OPTION<T> {
//   // Defining the generic option Type
//   Some(T), // represents a value
//   None, // represents the absence of a value
// }

// enum Result<T, E> {
//   // Defining the generic result Type
//   Ok(T), // represents a successful value
//   Err(E), // represents an error value
// }

fn main() {
    println!("Option hundling!");
    let result = divide(10.0, 2.0);
    match result {
        Some(value) => println!("Result = {}", value),
        None => println!("Error: Division by zero"),
    }

    println!("Result hundling!");
    let result2: Result<f64, String> = second_divide(32.32, 10.02);
    match result2 {
        Ok(value) => println!("Second result = {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

// using option type
fn divide (numerator: f64, denominator: f64) ->Option<f64>{
  if denominator == 0.0 {
    None
  } else {
    Some(numerator / denominator)
  }
}

fn second_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Error: Division by zero".to_string())
    } else {
        Ok(numerator / denominator)
    }
}