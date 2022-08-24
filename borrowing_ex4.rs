fn main() {
  let arr:[i32;4] = [1, 2, 3, 4]; // define an array
  let borrow_arr = &arr[0..2]; // slice an array

  println!("arr : {:?}", arr); // print the array
  println!("sliced_arr : {:?}", borrow_arr); // print the sliced array

  let str = String::from("Rust Programming"); // define a String object
  let borrow_str = &str[0..2]; // slice the String object

  println!("str : {:?}", str); // print the String Object 
  println!("sliced_str : {:?}", borrow_str); // print the sliced String

  let my_vec = vec![1, 2, 3, 4, 5]; // define a vector
  let borrow_vec = &my_vec[0..2]; // slice the vector
  
  println!("vec: {:?}", my_vec);  // print the vector
  println!("sliced_vec : {:?}", borrow_vec); // print the sliced vector
}
