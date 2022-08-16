/*
Rule No: 1#
If an item is public it can be accessed from anywhere, i.e., within main function or any other module.
*/

// declare a module
mod r {
  pub fn print_statement(){
    println!("Hi, this a function of module r");
  }
}
// main function
fn main() {
  println!("Let's go inside the module");
  // invoke a module 'r'
   r::print_statement();
}
