fn main(){


    let name = String::from("Rust");

    let new_name = name; // it's transfor the ownership 

    println!("{}", new_name); // it will print Rust 

    println!("{}", name);  // it will throw an error

    // To resolve this we can use the clone keyword 

    let new_name = name.clone(); // it's like get the mirror of the value

    // now we can print the both value without error 


}