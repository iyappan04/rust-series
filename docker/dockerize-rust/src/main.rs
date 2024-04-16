use std::collections::HashMap;

fn main() {
  
    println!("Check docker in Rust development.");

    let mut users = HashMap::new();

    users.insert(1, "iyappan");
    users.insert(2, "rahul");

    // println!("{:?}", users);

    match users.get(&2) {
        Some(value) => println!("Found with value = {}", value),
        None => println!("Not Found")
    }

}
