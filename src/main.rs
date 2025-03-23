// my first actual code in Rust

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the Number! You have 5 Tries");

    let tup: (i32,f64,u8, String) = (500, 23.01,1, String::from("Taku Is Awesome"));

    let mut tries = 5;
    let mut counter = 1;

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is: 93");

     while tries > 0 {
        
        println!("Enter your Number:");

        //need to put type checking here
        let mut guess = String::new();

        //to get user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the Line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(e) =>{
                println!("Enter an Actual Number Bro: {}", e);
                continue;
            }
        };

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{} is too Small", guess),
            Ordering::Greater => println!("{} Too Large", guess),
            Ordering::Equal => {
                println!("Well Done. You guessed Right!!");
                break;
            }
        } 

        println!("You Guessed {} number of times", counter);
        println!("You have {} number of tries left!!", (tries - 1));
        // println!("Random Shit: {}",  match counter{
        //     0 => tup.0.to_string(),
        //     1 => tup.1.to_string(),
        //     2 => tup.2.to_string(),
        //     3 => tup.3.clone(),
        //     _ => "Index out of bounds Bro".to_string(),
        // });
        tries -= 1;
        counter +=1;
        
    }
    if tries == 0{
        println!("Awewu bish. The secret number was: {} you dumbass",secret_number);
    }
}