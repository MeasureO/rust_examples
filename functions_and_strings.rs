fn main(){
   let course: &str = "Rust Programming";
   display_course_name(course); 
   println!("{}",course); // string literal is used after the function call

   let course:String = String::from("Rust Programming");
   display_course_name(course); 
   //cannot access course after display
}

// Passing Primitive String - String Literal
fn display_course_name(my_course: &str){
   println!("Course : {}", my_course);
}

// Passing Growable String - String Object
}
fn display_course_name(my_course:String){
   println!("Course : {}", my_course);
}
