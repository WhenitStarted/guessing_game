// use std::io;

fn main() 
{
    println!("Guess the number");

    println!("Input your number to guess");

    let mut guess = String::new();

    std::io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("You guessed: {}", guess);
}