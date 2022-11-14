fn prefix_print(prefix: String) -> impl Fn(&str) {
  move |suffix| println!("{prefix} {suffix}")
}

fn main() {
  let pp = prefix_print("Hello,".to_string());

  pp("World!");
}
