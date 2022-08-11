//define a tuple struct
struct FruitQuantity(String, i32);
// main function
fn main() {
    // create an instance
    let r1 = FruitQuantity("oranges".to_string(), 12);
    // access values of a tuple struct
    println!("r1--name:{} quantity:{}", r1.0, r1.1);
    // create an instance
    let r2 = FruitQuantity("mangoes".to_string(), 13);
    // access values of a tuple struct
    println!("r2--name:{} quantity:{}", r2.0, r2.1);   
}
