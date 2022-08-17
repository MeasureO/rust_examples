// If the module belonging to some other file is to be made accessible then it should be made public by using the pub keyword before the mod.
// Once the module is made public using pub, all privacy rules for defining modules within the same file apply.

// main.rs
mod my_mod; 
fn main() {
  println!("Invoke function in my_mod.rs");  
  my_mod::my_public_function();
}

// my_mod.rs
// declare a module
pub fn my_public_function() {
    println!("I am a public function in my_mod.rs");
}
