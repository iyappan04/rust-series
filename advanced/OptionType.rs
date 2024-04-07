// Option is genric enum type in rust 
// Used to knows the status of present and absent value 
// it will use in null concept


fn main(){


    let result = divide(10, 0);

    if result.is_none() {
        println!("Cannot divide by Zero");
    }
    else{
        println!("{}", result.unwrap());
    }


}

fn divide(dividend: i32, divisor: i32)-> Option<i32> {

    if divisor == 0 {
        return None
    }
    else{ 
        Some(dividend/divisor)
    }
        
}