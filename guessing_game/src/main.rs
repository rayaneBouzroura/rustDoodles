use core::num;
use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);    
    //println!("The secret number is {secret_number}");
    
    println!("input your guess :3 ");
    loop 
    {
        let mut guess = String::new();

        //reading the line and holding it on the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("failed reading the message");

        //turn the guessed num into an unsigned 32 bit integer
        //since the parse return either an ok or an err obj
        //we use pattern matching in order to skip the case
        //of the user giving us an input we cannot parse into a u32
        //such as this cute duck emoji 
        let guess: u32 =match guess.trim()
                              .parse() {
                                Ok(num) => num,
                                Err(_) => 
                                {println!("enter a valid number you dimwit");
                                continue;}
                              };
        println!("You guessed : {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Equal =>{
                println!("Woohoo you win");
                break;}
            Ordering::Greater => println!("Too big"),
        }    
    }
    

}
