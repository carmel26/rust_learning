#![allow(warnings)]

use std::collections::HashMap;
// working with Hash Map 

fn main() {
    let hello = "Здравствуйте"; // "Hello" in Russian
    println!("{}", hello);

    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);

    let team_name = String::from("Bob");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}