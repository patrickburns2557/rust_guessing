use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main()
{
    println!("Guess a number between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..=100); //creates a thread_rng object from rand, and executes gen_range function on it
    //println!("Secret number is: {}", secret_number);
    
    loop
    {
        println!("Input your guess:");

        let mut guess = String::new(); //mut makes the variable mutable

        io::stdin()
            .read_line(&mut guess) //calling read_line and passing in "guess" as the var to store the input in
            .expect("Failed to read line"); //error message to print if an issue occurs

        //go to next iteration of loop if a non-numerical value is input, ignoring any errors that .parse throws
        //otherwise, stores the converted number back into guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); //{} is a placeholder where you can tell the program to print variables
        //can also be written: println!("You guessed: {}", guess);

        //match the output of .cmp to it's corresponding result based on the input number
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small, try again."),
            Ordering::Greater => println!("Too large, try again."),
            Ordering::Equal =>  {
                println!("You win!");
                break; //break out of loop if correct answer found
            }
        }
    }
    
}