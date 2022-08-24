/* 
Rule # 1#
There can be either one mutable borrow or any number of immutable borrows within the same scope.

It is not possible to do a shared borrow as well as a mutable borrow operation simultaneously in the same scope.
If you want to do this in the same program code, then enclose a block of code within {}.
In the inner block perform the shared borrow, and in the outer block perform the mutable borrow. 
Vice versa, outer block perform the shared borrow, and in the inner block perform the mutable borrow.
*/
/// cannot mutable borrow b since its already a shared borrow
/// mutable borrow a in outer scope and shared borrow in inner scope
fn main() {
  
  let mut a = 1; // mutable variable a is defined
  println!("variable `a` :{}", a);
  let b = 1;
  println!("variable `b` :{}", b);
  {

      let r1 = &a; // no problem
      println!("variable `r1` references `a` in inner scope(SHARED BORROW(a)) :{}",r1);
      let r2 = &a; // no problem
      println!("variable `r1` references `a` in inner scope(SHARED BORROW(a) :{}",r2);
      println!("r1:{}\nr2:{}", r1, r2);
      // r1 and r2 scope end here
  }
  
  let r3 = &mut a; // no problem
  *r3 = 3;
  println!("variable `r1` references `a` in outer scope(MUTABLE BORROW(a) and derefernced it and changed value to 3) :{}",r3);
  let r4 = &b;
  println!("variable `r3` references `b` in outer scope(SHARED BORROW(b)) :{}",r4);
  let r5 = &b;
  println!("variable `r3` references `b` in outer scope(SHARED BORROW(b)) :{}",r5);
  println!("r3:{}\nr4:{}\nr5:{}", r3 , r4 , r5);
}
