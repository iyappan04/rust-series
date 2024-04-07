
use user::user_model;
use std::io::Read;
use std::fs::File;
use std::io::Write;

mod user;

fn main() {
    user_model();
    println!("Hello, world!");

    let mut file = File::open("./text.txt").expect("Couldn't found the file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Something went wrong on read file");

    println!("{}", content);

    let mut file_create = File::create("./output.txt").expect("Couldn't create the file");
    file_create.write_all(b"This file is created by Rust.").expect("Couldn't write the file");


}
