#[allow(unused_variables, unused_mut)]
fn main() {
  // define a String object
  let mut course = String::from("Rus");
  // push a character
  course.push('t');
  println!("This is a beginner course in {}.", course);

  // define a string object
  let mut course = String::from("Rust");
  // push a string
  course.push_str(" Programming");
  println!("This is a beginner course in {}.", course);

  // define a String object 
  let course = "Rust".to_string();
  // define a String object
  let course_type = " beginner course".to_string();
  // concatenate using the + operator
  let result = course + &course_type;
  println!("{}", result);

  let course = "Rust".to_string();
  let _course_type = "beginner course".to_string();
  // default format macro 
  let result = format!("{} {}", course, _course_type);
  println!("{}", result);
  // passing value in the placeholder in the format macro 
  let result = format!("{1} {0}", course,_course_type);
  println!("{}", result);

}
