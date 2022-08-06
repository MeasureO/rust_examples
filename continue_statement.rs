fn main() {
  // define a for loop
  for var in 0..10 {
     if var == 4 {
        println!("I encoutered a continue statement");
        continue;
      }
      println!("var: {}", var);
      println!("I did not encounter continue statement");
  }

    // define an integer variable
    let mut var = 1; 
    // define a boolean variable
    let mut found = false;
    // define a while loop
    while !found {
      var = var + 1;
      println!("{}", var);
      
      if var == 4 {
          println!("I encoutered a continue statement");
          continue;
        }
        println!("I did not encounter continue statement");
        
        if var == 10{
          found = true;
        }
    }

  // define an integer variable
  let mut var = 1; 
  // define a loop
  loop {
    var = var + 1;
    println!("{}", var);
    
     if var == 4 {
        println!("I encoutered continue statement");
        continue;
      }
      println!("I did not encounter continue statement");
  }
}
