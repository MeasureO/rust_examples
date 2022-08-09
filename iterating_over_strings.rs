fn main() {
  // define a String object
  let str = String::from("Rust Programming"); 
  // split on whitespace
  for token in str.split_whitespace(){
      println!("{}", token);
  }

  // define a String object
  let str = String::from("Educative, course on, Rust, Programming");  
  // split on token
  for token in str.split(","){
      println!("{}", token);
  }

  // define a String object
  let str = String::from("Rust Programming");  
  // split on literal
  for token in str.chars(){
      println!("{}", token);
  }
}
