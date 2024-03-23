use std::io;


fn main(){

    println!("Please Input");

    let mut input = String::new();

    io::stdin().read_line(&mut input);

    println!("You Typed {}", input);
    
}