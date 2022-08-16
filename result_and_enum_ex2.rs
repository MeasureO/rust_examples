fn main() {
   println!("{:?}", divisible_by_3(6)); // invoke function by passing a number 6
   println!("{:?}", divisible_by_3(2)); // invoke function by passing a number 2
}
fn divisible_by_3(i:i32)->Result<String,String> {
   if i % 3 == 0 { // if number mod 3 equals 0
      Ok("Given number is divisible by 3".to_string()) // return this statement
   } else { // if if number mod 3 is not equals 0
      Err("Given number is not divisible by 3".to_string()) // return this statement
   }
}
