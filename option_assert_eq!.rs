fn main() {
    let my_val: Option<&str> = Some("Rust Programming!");
    // pass since my_val is set to some value so left is true, and right is also true
    assert_eq!(my_val.is_some(), true); 
    // pass since my_val is set to some value so left is false, and right is also false
    assert_eq!(my_val.is_none(), false);
}
      
     // Assert Macros

// assert_eq!(left, right) - evaluates to true if left value is equal to that of right
// assert_ne!(left, right) - evaluates to true if left value is not equal to that of right
