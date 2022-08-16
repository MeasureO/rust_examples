Rule No: 2#
If an item is private it can be accessed using its parent module meaning it can be accessed within the module but not outside it.

// declare a module
mod r{
  fn my_private_function(){
    println!("Hi, I'm a private function within the module");
  }
  pub fn my_public_function(){
    //! also works without writing self i.e.
    //! my_private_function();
    println!("Hi,I'm a public function within the module");
    println!("I'll invoke private function within the module");
    self::my_private_function(); 
    
  }
}
// main function
fn main() {
  println!("Let's go inside the module");
  // invoke a module 'r'
   r::my_public_function();
}
