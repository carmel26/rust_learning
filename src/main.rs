fn main() {
    // compound data types
    // arrays, tuples, slices, structs, enums and Strings
    
    //  ===== arrays =====
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Th enumber in our array are: {:?}", numbers);
    // print an error 
    // let mix = [1, 2, 4, 'Apple', 3.13];
    // println!("The mixed array is: {:?}", mix);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("The fruits in our array are: {:?}", fruits);
    println!("The first fruit in our array is: {}", fruits[0]);

    // ===== tuples =====
    let person: (String, i32, f64, bool) = ("John Paul".to_string(), 30, 164.9, true);
    println!("The person is: {:?}", person);
    let mixed_tuple = ("Kratos", 30, 164.9, true, numbers);
    println!("The mixed tuple is: {:?}", mixed_tuple);

    // ==== slices =====
    let slice: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("the slice is equal to: {:?}", slice);
    let animals: &[&str] = &["Dog", "Cat", "Bird", "Fish", "Rabbit"];
    println!("the animals in our slice are: {:?}", animals);

    // string vs Strings
    let mut stone_cold: String = String::from("Hello, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says : {}",stone_cold);

    // B-&str
    let string : String = String::from("Bonjour tout le monde");
    let slice: &str = &string;
    println!("The slice is : {}", slice);

}
