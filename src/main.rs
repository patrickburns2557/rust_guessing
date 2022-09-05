use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main()
{
    println!("Guess a number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100); //creates a thread_rng object from rand, and executes gen_range function on it
    println!("Secret number is: {}", secret_number);

    println!("Input your guess:");

    let mut guess = String::new(); //mut makes the variable mutable

    io::stdin()
        .read_line(&mut guess) //calling read_line and passing in "guess" as the var to store the input in
        .expect("Failed to read line"); //error message to print if an issue occurs

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //convert guess from string to unsigned 32-bit int, w/ error message if not able to
    //"shadows" the previous declaration of guess, to allow us to reuse the "guess" name w/o needing a new variable name

    println!("You guessed: {guess}"); //{} is a placeholder where you can tell the program to print variables
    //can also be written: println!("You guessed: {}", guess);


    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too small, try again."),
        Ordering::Greater => println!("Too large, try again."),
        Ordering::Equal => println!("You win!"),
    }
}