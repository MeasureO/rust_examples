//bad 

#![allow(unused)]
fn main() {
struct FactorialResult {
    valid: bool,
    value: i32,
}
fn factorial(n: i32) -> FactorialResult {
    // Assumes n is non-negative.
    fn f(n: i32) -> i32 {
        if n == 0 { 1 } else { n * f(n - 1) }
    }
    let valid = n >= 0;
    let value = if valid { f(n) } else { n };
    FactorialResult {
        valid,
        value,
    }
}
}
// not bad

#![allow(unused)]
fn main() {
enum FactorialResult {
    Valid(i32),
    InvalidArgument(i32),
}
fn factorial(n: i32) -> FactorialResult {
    // Assumes n is non-negative.
    fn f(n: i32) -> i32 {
        if n == 0 { 1 } else { n * f(n - 1) }
    }
    if n >= 0 {
        FactorialResult::Valid(f(n))
    } else {
        FactorialResult::InvalidArgument(n)
    }
}
}

// good 


#![allow(unused)]
fn main() {
fn factorial(n: i32) -> Result<i32, i32> {
    fn f(n: i32) -> i32 {
        if n == 0 { 1 } else { n * f(n - 1) }
    }
    if n >= 0 {
        Ok(f(n))
    } else {
        Err(n)
    }
}
}
        
        
            
