fn main(){

    match divide(9.0, 3.0) {
        Ok(result) => println!("Result = {}", result),
        Err(ex) => println!("Error is = {}", ex)
    }

}


fn divide(dividend: f32, divisor: f32) -> Result<f32, String> {
    if divisor == 0.0 {
        Err(String::from("Error cannot divide by zero"))
    }
    else {
        Ok(dividend/divisor)
    }
}