const ID_1: i32 = 4; // define a global constant variable
fn main() {
    const ID_2: u32 = 3; // define a local constant variable
    let mut language = "Rust"; // define a mutable variable
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable

    let (course,category) = ("Rust","beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
    

  let outer_variable = 112;
  {
        let inner_variable = 213;
        println!("block variable inner: {}", inner_variable);
        let outer_variable = 117; // SHADOWING
        println!("block variable outer: {}", outer_variable);
  }
    // ERROR: println!("inner variable: {}", inner_variable); // use of inner_variable outside scope

}
