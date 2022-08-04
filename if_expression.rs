fn main() {
      //define a variable 
      let learn_language="Rust";
      // if..elseif..else construct 
      if learn_language == "Rust" { 
         println!("You are learning Rust language!");
      }
      else if learn_language == "Java" { 
         println!("You are learning Java language!");
      }
      else {
         println!("You are learning some other language!");
      } 
      // shorthand if
      let res= if learn_language == "Rust" {"You are learning Rust language!"} else {"You are learning some other language!"};
      println!("{}", res);
      // return value
      let y: bool = if x == "Rust" { true } else { false };

      // ERROR
      // let z: bool = if x == "Rust" { true; } else { false; };

}
