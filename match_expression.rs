fn main() {
    // define a variable 
    let x = 5;
    // define match expression
    match x {
        1 => println!("Java"),
        2 => println!("Python"),
        3 => println!("C++"),
        4 => println!("C#"),
        5 => println!("Rust"),
        6 => println!("Kotlin"),
        _ => println!("Some other value"),
    };

   // define a variable
   let course = "Rust";
   // return value of match expression in a variable
   let found_course = match course {
      "Rust" => "Rust",
      "Java" => "Java",
      "C++" => "C Plus Plus",
      "C#" => "C Sharp",
      _ => "Unknown Language"
   };
   println!("Course name : {}",found_course);
}
