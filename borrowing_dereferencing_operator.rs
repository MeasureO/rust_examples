fn main() {
    let x = 10;
    let mut y = 13;
    //immutable reference to a variable
    let a = &x;
    println!("Value of a:{}", a); 
    println!("Value of x:{}", x); // x value remains the same since it is immutably borrowed
    //mutable reference to a variable
    let b = &mut y;
    println!("Value of b:{}", b);

    *b = 11; // derefencing 
    println!("Value of b:{}", b); // updated value of b
    println!("Value of y:{}", y); // y value can be changed as it is mutuably borrowed
    
    
    
        //mutable reference to a variable
    let mut x = 10;
    println!("Value of x:{}", x);
    let a = & mut x;
    println!("Value of a:{}", a);
    //dereference a variable
    *a = 11;
    println!("Value of a:{}", a);
    println!("Value of x:{}", x); // Note that value of x is updated
}
