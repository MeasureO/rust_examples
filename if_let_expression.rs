fn main() {
    // EXAMPLE 1
    // define a scrutinee expression    
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner","course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }


    // EXAMPLE 2
    // define a scrutinee expression    
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", c) = course {
        println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    } 
    else {
        // do not execute this block
        println!("Value unmatched");
    }

    // EXAMPLE 3
    // define a scrutinee expression     
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", c, d) = course {
        println!("Wrote one value in pattern to be matched with the scrutinee expression.Guessed values: {}, {}",c,d);
    } else {
        // do not execute this block
        println!("Value unmatched");
    }
    // EXAMPLE 4

    // define a scrutinee expression     
    let course = ("Rust", "beginner");
    // pattern does not match with the scrutinee expression
    if let ("Java", c) = course {
        println!("Course is {}", c);
    } else {
        // execute this block
        println!("Value unmatched");
    }
    // EXAMPLE 5
    // no pattern is define
    // WARNING
    if let _ = 10 {
        println!("irrefutable if-let pattern is always executed");
    }
}
