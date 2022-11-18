// doesn`t compile
struct MyStruct<A, B> {
    a: A,
    b: B,
}
impl<A, B> MyStruct<A, B> {
    fn print(&self) {
        println!("a: {}, b: {}", self.a, self.b);
    }
}
  
// and that`s okay
use std::fmt::Display;

impl<A: Display, B: Display> MyStruct<A, B> {
    fn print(&self) {
        println!("a: {}, b: {}", self.a, self.b);
    }
}
// and you can use this
impl<A, B> MyStruct<A, B>
where
    A: Display,
    B: Display {
