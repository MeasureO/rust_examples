fn main() {
    let mut language = "Rust"; // define a mutable variable
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable

    let (course,category) = ("Rust","beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
}
