fn main() {
    println!("Working with ownership!");  
    // ============================ cleaning the memory ============================
    //  example each value in rust has a variable that's its owner 

    let s1 = String::from("Rust");
    let len = calculate_length (&s1);
    println!("The length of '{}' is {}.", s1, len);

    // example of there can be only be one owner at a time means we send the ownership of s1 to s2
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    println!("The length of '{}' is {}.", s2, len);
}


    // when the owner goes out of scope , the value will be dropped and the memory will be freed automatically
    fn print_lost(s : &String){
        // display an error 
        // println!("{}",&s1)
    }

fn calculate_length(s: &String) -> usize {
    s.len()
}
