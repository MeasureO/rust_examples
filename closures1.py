fn main() {
  let a = [1, 2, 3];
  let n: i32 = a.iter().map(|x| x * 2).sum();
  println!("Sum of {:?} after doubling: {}", a, n);
}
