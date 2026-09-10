#![allow(warnings)]

// working with vectors

fn main() {
  // defining an empty vector
   let mut vector: Vec<i32> = Vec::new();

   vector.push(12);
   vector.push(23);
   vector.push(1);
   vector.push(4);
   vector.push(5);

   println!("The first vector is {:?}", vector);

   // defining a vector with values
   let theVec : Vec<i32> = vec![12, 23, 34, 12, 32, 43, 32];

   //by reference
   let the_second_element : &i32 = &theVec[1];
   println!("The second element of the second vector is {}", the_second_element);

   // by value
   let third_element = theVec[2];
   println!("The third element of the second vector is {}", third_element);

   //USING Get metho
    match theVec.get(3) {
        Some(third_element) => println!("The fourth element of the second vector is {}", third_element),
        None => println!("There is no fourth element in the second vector"),
    }

}