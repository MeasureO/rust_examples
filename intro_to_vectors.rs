fn main() {
  //define a vector of size 4
  let my_vec = vec![1, 2, 3, 4, 5];
  //let my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
  //print the vector
  println!("{:?}", my_vec);

  //access a particular value
  println!("{}", my_vec[0]);

  match my_vec.get(9) {
      Some(x) => println!("Value at given index:{}", x),
      None => println!("Sorry, you are accessing a value out of bound")
  }
  //using debug trait 
  println!("Vector : {:?}", my_vec);
  println!("Print using for loop"); 
  // using loop
  let mut index = 0;
  for i in my_vec {
      println!("Element at index {}:{} ", index, i);
      index = index+1;
  }
  // methods
  let mut my_vec = Vec::new();
  println!("Empty Vector : {:?}", my_vec);
  my_vec.push(1);
  my_vec.push(2);
  my_vec.push(3);
  println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
  my_vec.pop();
  println!("Popped value: {}", 3);
  println!("Popped element at last index : {:?}", my_vec);
  my_vec.remove(1);
  println!("Removed value: {}", 2);
  println!("Removed element at index 1 : {:?}", my_vec);
  println!("Size of vector is :{}", my_vec.len());
  println!("Does my vector contains 1 : {}", my_vec.contains(&1));
}
