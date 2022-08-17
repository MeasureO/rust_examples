fn main() {
    let a = 1;
    let b = a; // copy of 'a' is created
    println!("a:{} , b:{}", a, b); // print 'a' and 'b'
    // Note: The value of a is copied to b

 
    let a = [1,2,3];
    let b = a; // copy of 'a' is created 
    println!("a:{:?} , b:{:?}", a, b); // print 'a' and 'b'
    // Note: The value of a is copied to b

    // The ownership state of the original variable (whose value is assigned to another variable) is set to moved.
    // This means that the original variable binding cannot be accessed.

    let a = String::from("Rust");
    let b = a; // moves value of 'a' to 'b'
    eprintln!("a:{} , b:{}", a, b); // Error use of moved value 'a'
    // Note: The following code gives an error, ❌, since the value of a is moved to b and a becomes inaccessible.

    let a = vec![2, 4, 8];
    let b = a; // move value of 'a' to 'b'
    println!("b : {:?} ", b); // prints 'b'
    //println!("{:?} {:?}", a, b); // Error; use of moved value: 'a'
    // Note: The following code gives an error, ❌, if the commented statement is uncommented since the value of a is moved to b and a becomes inaccessible.

    
    let mut a = String::from("Rust"); // define a String and save in 'a'
    let b = a.clone(); // b clones a
    a.push('y');
    println!("a:{} , b:{}", a, b);  // print 'a' and 'b'


}
