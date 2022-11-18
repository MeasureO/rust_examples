// derived debug
println!("{:?}", "Hello");
println!("{:?}", vec!["Hello", "World"]);

// now you need to turn on it
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

let mick = Person {
    name: "Mick".to_string(),
    age: 30
};

println!("{:?}", mick);
