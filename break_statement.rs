fn main() {
  // define a for loop
  for i in 0..10 {
    println!("i:{}", i);
    if i == 5 {
      break;
    }
  }

  let mut i = 1;
  let found = false;
  // define a while loop
  while !found {
    println!("i:{}", i);
    if i == 5 {
      break;
    }
    i = i + 1;    
  }
  
  let mut i = 1;
  // define a loop
  loop{
    println!("i:{}", i);
    if i == 5 {
      break;
    }
    i = i + 1;    
  }
}
