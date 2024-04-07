use std::io;

fn main() {
    println!("guss the number!");
    println!("plese input your guess. ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line"); // error handling
 println!("you guessed: {}", guess);
}