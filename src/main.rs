#![allow(warnings)]
// structures are used to name and package related values similar to tuples but with more flexibility
  // creating a book structure
  struct Book {
    title: String,
    author: String,
    number_of_pages: u32,
    available: bool,
  }

  // creating a user structure
  struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
  }

fn main() {
  // advanced structures understandings
  let rect  = (30, 50);
  
  let mut user1 : User = User {
    active : true,
    username : String::from("carmel1"),
    email : String::from("carmelnkeshi@gmail.com"),
    sign_in_count : 1,
  };

//   changing the email of user1 using the object
  user1.email = String::from("carmelnkeshi2@gmail.com");
  println!("User email: {}", user1.email);
  
  let user2 : User = User {
    email : String::from("mynewEmail@gmail.com"),
    ..user1
  };
  println!("User email: {}", user2.email);

  // creating a tuples struct
  struct Color(i32, i32, i32);
  struct Point(i32, i32);   

  let black = Color(0, 0, 0);
  let white = Color(255, 255, 255);

  // unit-like struct
  struct AlwaysEqual;
  let subject: AlwaysEqual = AlwaysEqual;


}

fn build_user(email: String, username:String) -> User {
    User {
      active : true,
      username,
      email,
      sign_in_count : 1,
    }
  }