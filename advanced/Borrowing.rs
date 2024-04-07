fn main(){


 
    let mut namec = String::from("Rust Borrow");

    // we can create many immutable borrowing in rust. 
    // let mut name = String::from("Rust Borrow");
    let new_name1 = &namec; // immutable / readonly access 
    let new_name2 = &namec; // immutable / readonly access 
     

    // we can create only on mutable reference at a time 

    let mut name = String::from("Rust Borrow");
    let change_name1 = &mut name; // We can't use mutable reference twice it will throw in error.
    change_name1.push_str("Somethig is good"); 
    let change_name2 = &mut name;  // Here only will access the value 
    change_name2.push_str("Somethig is good");

    println!("name 2 {} ", change_name1);


    // Also we can't use the mut and immutable reference at a time 
    

}