fn main() {
   let check1 = divisible_by_3(6);
   assert_eq!(check1.is_ok(), true);  // left is true and right is true so the assertion passes
   let check2 = divisible_by_3(2);
   assert_eq!(check2.is_err(), true); // left is true and right is true so the assertion passes
}
fn divisible_by_3(i:i32)->Result<String,String> {
   if i % 3 == 0 {
      Ok("Given number is divisible by 3".to_string())
   } else {
      Err("Given number is not divisible by 3".to_string())
   }
}
