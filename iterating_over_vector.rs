fn main() {
  // defines a mutable vector
  let mut my_vec = vec![1, 2, 3, 4, 5];
  // define the value to be removed
  let value = 2; 
  // get the index of the value in the vector
  let index = my_vec.iter().position(|&r| r == value).unwrap();
  // call the built-in remove method
  my_vec.remove(index);
  // print the updated vector
  println!("Updated Vector: {:?}", my_vec);

  // using loop
  let mut index = 0;
  for i in my_vec.iter(){ // it works even if .iter() is not written
      println!("Element at index {}:{} ", index, i);
      index = index + 1;
  }

  println!("Initial Vector : {:?}", my_vec);
  for x in my_vec.iter_mut(){
     *x *= 3;
  }
  // print the updated vector
  println!("Updated Vector : {:?}", my_vec);
}
