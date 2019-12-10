use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    println!("Please enter your guess for the number: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    println!("You guessed {}", guess);
} // end main()
