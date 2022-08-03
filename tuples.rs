#[allow(unused_variables, unused_mut)]
fn main() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // access value of a tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);
    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);

    // define a tuple with type annotated
    let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");
    let (w, x, y, z) = person_data;
    //print values
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Weight : {}", y);
    println!("Height : {}", z);
}
}
