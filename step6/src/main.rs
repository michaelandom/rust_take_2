use rand::Rng;
use std::io;
use core::cmp::Ordering;
use colored::*;
fn main() {
    println!("Guessing Game config");
    let min: i32=range_number(1, String::from("Enter Min range for the number"),None);
    let max: i32=range_number(100, String::from("Enter Max range for the number"),Some(min));
    let count: i32=range_number(3, String::from("Enter Max life for the game"),Some(1));

    game(max,min,count); 


}
fn range_number(default: i32, title: String, min:Option<i32>) -> i32 {
    println!("");
    println!("{} ({})",title,default);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("OS Error");
    let value = match input.trim().parse() {
        Ok(num) => {
          match min {
            Some(min_val) if num < min_val => {
                println!("Input must be greater than or equal to {}", min_val);
                println!("system selected {}", min_val + 10);
                return min_val + 10;
            }
            _ => num,
          }    
        },
        Err(_) => {
            println!("Incorrect Input");
            100
        }
    };
    value
}

fn game(max:i32, min:i32, count:i32){
    println!("Guessing Game");
    let secret_number = rand::thread_rng().gen_range(min, max);
    println!("Selected number is {}", secret_number);
    let mut count = count;

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess).expect("Os Error");
        println!("You guessed: {}", guess);
        let guess: i32 = match guess.trim().parse()  {
         Ok(number) => number,
         Err(_) => {
            println!("Please Enter only Number!!");
            continue
         } 
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
               break;
            },
        }

        count = count -1;
        if count <= 0 {
            println!("{} {}","you lost the game the number was".red(),secret_number);
            break;   
        }
    }
}
