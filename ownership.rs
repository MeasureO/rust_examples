fn main() {
  let a = 1; // variable a is the owner of the value 1
  let b = 1; // variable b is the owner of the value 1
  let c = 3; // variable c is the owner of the value 3
  
  println!("a : {}", a);
  println!("b : {}", b);
  println!("c : {}", c);
}// value a, b, c are out of scope outside this block
