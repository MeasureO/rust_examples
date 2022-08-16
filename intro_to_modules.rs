// declare a module
mod r {
  fn print_statement(){
    println!("Hi, this a function of module r");
  }
}
// main function
fn main() {
  // invoke a module 'r'
   r::print_statement();
 // ERROR
// r is private
}
