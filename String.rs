
fn main(){
    
    let mut text = String::from("Hello, I am a Rust developer");
    println!("{}", text);
    
    for token in text.split(" "){
        println!("{}", token);
    }

    text.push_str(", Doing code in rust");

    println!("{}", text);

    println!("{}", text.contains("Doings"));

    println!("{}", text.len());

}