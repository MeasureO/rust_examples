// Explicit declaration

// main.rs

mod my_mod; 
fn main() {
  println!("I am a public function in my_mod.rs");
  my_mod::module::my_public_function();
}

// my_mod.rs

// declare a module
pub mod module{
pub fn my_public_function() {
    println!("I am a public function in my_mod.rs");
}
}
