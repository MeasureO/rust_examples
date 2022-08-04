fn main() {
    let a = 15;
    let b = (a as f64) / 2.0; 
    println!("a: {}", a);
    println!("b: {}", b);
    /* THIS IS NOT WORKING
    let a: char = 'r' ; // cannot be type casted
    let b = a as &str ; 
    println!("a: {}", a);
    println!("b: {}", b);
    */
}
