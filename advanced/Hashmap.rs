use std::collections::HashMap;



fn main() {
    let mut users = HashMap::new();

    users.insert(1, "iyappan");
    users.insert(2, "rahul");

    // println!("{:?}", users);

    match users.get(&2) {
        Some(value) => println!("Found with value = {}", value),
        None => println!("Not Found")
    }

    // println!("{}", users.contains_key(1));

    // users.remove(1);
    // println!("{:?}", users);

}