extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guss the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
   // println!("the secret number is: {}", secret_number);  //print out the secret number)
   let mut attampts = 0;
   let max_attampts = 12;
    loop {
        
    
    println!("plese input your guess. ");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)

    .expect("Failed to read line"); // error handeling

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,   // to continue if input is not a number
        };
        
         

 println!("you guessed: {}", guess);
match guess.cmp(&secret_number) {

    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => { 
    println!("You win bro!");
    break;
    }

}

attampts += 1;
if attampts == max_attampts {
    println!("You lost bro 🤡");
    break;
}
    
}    

}