// The example below shows how the root function (a function that exists outside the module)
// can be accessed within the function of a module. Write super:: followed by the root function name.

// main function
fn main() {
  println!("Let's go inside the module");
  my_module ::my_public_function();
}
fn my_function(){
  println!("Hi, you came inside the root function using super");
  }

// declare a module
mod my_module {
  // function within outer module
  pub fn my_public_function() {
    println!("Invoke root function");
    super::my_function();
  }
}
