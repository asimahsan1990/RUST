use std::io;
use rand::Rng; 
use std::cmp::Ordering;

fn main() {
    println!("Guess a Number");
   
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
    let mut guess = String::new();
    println!("Enter the a number");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less=>println!("too small"),
        Ordering::Greater=>println!("too big"),
        Ordering::Equal=>{
                        println!("you win!");
                        break;
                    }
        }
    }
}
