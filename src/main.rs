use std::io; // import io library - this is called 'bringing it into scope' from the standard library - std, like import io from std in python
use std::cmp::Ordering;
use rand::Rng;

fn main() { // start main function - entry point for program
    println!("Guess the number!"); // like python print() statement, but its a macro

    let secret_number = rand::thread_rng().gen_range(1..=100);
        
    loop {
        println!("Please input your guess here."); // same as above 
        
        let mut guess = String::new();  // declare a new mutable variable called 'guess' and make it of type String using the 'new' function from the String part of the standard library
        
        io::stdin() // get the user input, seems like this is like python's input() function
            .read_line(&mut guess) // put the user input into the 'guess' variable
            .expect("failed to read line");  // handle errors if there is a problem
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            },
        };

        println!("You guessed: {guess}"); // output content of guess variable, like a python fstring
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too low!"),
            Ordering::Greater => println!("Your guess was too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



    

}