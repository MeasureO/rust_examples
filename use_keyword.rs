pub mod chapter {
    pub mod lesson { // mod level 1
        pub mod heading { // mod level 2
            pub fn illustration() {  // mod level 3
              println!("Hi, I'm a 3rd level nested function");
            }
        }
    }
}
use chapter::lesson::heading; // make use of `use` keyword
fn main() {
    heading::illustration(); // call the function
}
