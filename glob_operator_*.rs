// make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum KnightMove{
   Horizontal,Vertical
}

use KnightMove::*; // use of globe operator
fn main() {
   // use enum
   let horizontal_move = Horizontal; // Horizontal is shortcut for KnightMove::Horizontal
   let vertical_move = Vertical; // Vertical is shortcut for KnightMove::Vertical
   // print the enum values
   println!("{:?}", horizontal_move);
   println!("{:?}", vertical_move);
}
