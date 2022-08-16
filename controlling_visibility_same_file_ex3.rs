If an item is private, it can be called from within the child module.

// main function
fn main() {
  println!("Let's go inside the module");
  outer_module::inner_module::my_public_function();
}
// declare a module
mod outer_module {
  // function within outer module
  fn my_private_function() {
    println!("Hi, I got into the private function of outer module");
  }
  // declare a nested module
  pub mod inner_module {
    // function within nested module
    pub fn my_public_function() {
      println!("Hi, I got into the public function of inner module");
      println!("I'll invoke private function of outer module");
      super::my_private_function();
    }
  }
}

// Even though the function my_private_function() is declared private, 
// the main() function is able to invoke it indirectly because the function it calls is public.
