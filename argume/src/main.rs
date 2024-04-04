use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];
    let len = &args[3];


    println!("Searching for {}", query);
    println!("In file {}", file_path);
    println!("Times to execute {}", len);
}
