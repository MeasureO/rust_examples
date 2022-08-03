#[allow(unused_variables, unused_mut)]
fn main() {
   //define an array of size 4
   let arr:[i32;4] = [1, 2, 3, 4]; 
   //print the first element of array
   println!("The first value of array is {}", arr[0]);
   // print array
   println!("Array: {:?}", arr);
   // length of array
   println!("Length of array: {}", arr.len());

   // initialize an array of size 4 with 0
   let arr1 = [0 ; 4]; 
   //print the first element of array
   println!("The first value of array is {}", arr1[0]);

    //define a mutable array of size 4
    let mut arrm:[i32;4] = [1, 2, 3, 4]; 

    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //define the slice
    let slice_array1:&[i32] = &arr;
    let slice_array2:&[i32] = &arr[0..2];
    // print the slice of an array
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);

}
