/*
Rule # 2
References must always be valid.

Cannot reference a value that is moved, i.e., a non-primitive data type.
*/
fn main() {

  let a = String::from("Rust"); //variable a

  println!("This is a variable a: {}", a);

  let b = a; // moves value of a to b

  println!("Value of variable a is moved to b.\n b : {}", b);
  println!("Now a becomes invalid.Accessing a will give error");
  
  //let c = &a;
  //println!("This is a variable c trying to access value a: {}", c);
}
