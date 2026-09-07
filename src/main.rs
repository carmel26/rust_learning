fn main() {
    println!("Working with functions!");  
    let sum = add(5, 10);
    println!("Sum: {}", sum);
    human_id("John Elan", 30, 175.9);

    // creating an expression which return the product of price and quantity
    let x= {
        let price = 5;
        let quantity = 10;
        price * quantity
    };
    println!("Total Price: {}", x);

    // calling the BMI function
    let height_m = 1.75;  
    let weight_kg = 70.0;  
    let bmi = calculate_bmi(weight_kg, height_m);
    println!("BMI: {:.2}", bmi);
}



// functions that return a value
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn human_id(name: &str, age: u32, height: f32){
    println!("Name: {}, Age: {}, Height: {} cm", name, age, height);
}

fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg / (height_m * height_m)
}