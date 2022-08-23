fn main() {
    let str = String::from("Rust"); // str comes into scope
                                    // str is a move type

    pass_string_object(str);        // str's value moves into the function...
                                    // ... and becomes in accessible here
    //println!("{}" , str);         // This line will give an error

    let my_int = 10;                // my_int comes into scope

    pass_integer(my_int);          // my_int value is a copy into the function,
                                    // but i32 is a copy type, so can my used
                                    // use my_int if desired

} // Here, my_int and then str goes out of scope

fn pass_string_object(my_string: String) { // my_string comes into scope
    println!("{}", my_string);
} // Here, my_string goes out of scope and `drop` frees the memory

fn pass_integer(my_integer: i32) { // my_integer comes into scope
    println!("{}", my_integer);
} // Here, my_integer goes out of scope
