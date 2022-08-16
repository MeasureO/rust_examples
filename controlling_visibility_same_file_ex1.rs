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
