use std::{cmp::Ordering, io};

use rand::{rngs, Rng};

fn main() {

    // let mut random_generator = rand::thread_rng();
    // let num: i32 = random_generator.gen_range(1..10);

    // println!("Genarated Number is {}", num);
    // println!("Hello, world!");

    // Guessing game in rust 

    let mut random_generator = rand::thread_rng();
    let number: i32 = random_generator.gen_range(1..20);

    let mut user_input = String::new();

    loop {

        println!("Please Guess:");

        let _ = io::stdin().read_line(& mut user_input);

        let input = user_input.trim().parse::<i32>().unwrap_or_else(|_| -1);

        match number.cmp(&input) {
            Ordering::Greater => println!("Number is greater"),
            Ordering::Less => println!("Number is less"),
            Ordering::Equal => {
                println!("Number is equal {} {}", user_input, number);
                break;
            }
        }

        user_input.clear();
        
    }

}
