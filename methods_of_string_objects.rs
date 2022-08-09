fn main() {
  // define a growable string variable
  let course = String::from("Rust");
  println!("This is a beginner course in {}.", course);
  //capacity in bytes
  println!("Capacity: {}.", course.capacity());
  // Note: The length of String will always be less than or equal to the capacity.

  // define a growable string variable
  let str = String::from("Rust Programming"); 
  let sub_str = String::from("Rust"); 
  println!("This is a beginner course in {}.", str);
  // find if string contains a substring
  println!("{} is a substring of {}: {}.", sub_str, str, str.contains(sub_str));

  // define a growable string variable
  let str = String::from("Rust Programming"); 
  let replace_from = "Programming";
  let replace_to = "Language"; 
  // find if string contains a substring
  let result = str.replace(replace_from, replace_to);
  println!("{} now becomes {}.", str, result);

  let string = "   Rust     Programming     ".to_string();
  let trim_string = string.trim(); 
  // get characters at 5,6,7,8,9,10 and 11 indexes
  println!("Trimmed_string : {}", trim_string);

  
}
