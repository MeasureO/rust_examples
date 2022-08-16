fn main() {
   let check1 = divisible_by_3(6);
   if check1.is_ok(){ // check if the function returns ok
      println!("The number is divisible by 3");
   }
   else{
      println!("The number is not divisible by 3");

   }
   let check2 = divisible_by_3(2);
   if check2.is_err(){ // check if the function returns error
      println!("The number is not divisible by 3");
   }
   else{
      println!("The number is divisible by 3");

   }
}
fn divisible_by_3(i:i32)->Result<String,String> {
   if i % 3 == 0 { // check i modulus 3
      Ok("Given number is divisible by 3".to_string())
   } else {
      Err("Given number is not divisible by 3".to_string())
   }
}
