fn main() {

    //define a for loop 
    for i in 0..5 {
      println!("{}", i);
    }
   // enumerate
  for (count, variable) in (7..10).enumerate() {
      println!("count = {}, variable = {}", count, variable);
  }
}
