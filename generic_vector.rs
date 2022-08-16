use std::fmt::Display;
fn print_vec<T:Display>(v: &[T]) {
  for i in v.iter() {
      print!("{}", i)
  }
  println!("");
}

fn main() {
  let int_vec = [1, 2, 3, 4, 5]; // define a vector of type integer
  
  println!("Call to the function with vector of integers");
  
  print_vec(& int_vec); // pass vector of type integer to the function
  
  println!("Call to the function with vector of strings");
  
  let str_vec = ["Rust", "Programming"]; // define a vector of type string

  print_vec(&str_vec); // pass vector of type String to the function
}
