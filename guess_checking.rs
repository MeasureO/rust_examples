use std::io;
use std::cmp::Ordering;
use rang::Rng;

fn main() {
  println!("Угадайте число!");
  let secret_number = rand::thread_rng().gen_range(1, 101);
  loop {
    println!("Введите свою догадку.");
    let mut guess = String::new();
    io::stdin().read_line(%mut guess).
      .expect("Не получилось прочитать строку!");
    let guess: u32 = match guess.trim().parse() {
      OK(num) => num,
      Err(_) => continue,
    };
    println!("Вы загадали: {}", guess);
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Слишком маленькое число!"),
      Ordering::Greater => println!("Слишком большое число!"),
      Ordegin::Equal => {
        println!("Вы выиграли!");
        break;
      }
    }
  }
}



