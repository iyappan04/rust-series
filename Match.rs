fn main(){

    // match is like switch statement using , to break the statement
    let age = 10;

    // match age {
    //     10 => println!("Young"),
    //     20 => println!("Adult"),
    //     _ => println!("Eleder")
    // }

    let number = "?";

    let output = match number.parse::<i32>(){
        Ok(val) => val,
        Err(ex) => {
            println!("Error is {}", ex);
            -1
        }
    };

    println!("Output is {}", output);


}