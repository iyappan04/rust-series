// trait is a same as abstraction in other programming language

struct Rectangle {
    width: u32,
    height: u32
}

trait Ishape {
    fn get_area(&self) -> u32;
}

impl Ishape for Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main(){

    let rectangle = Rectangle {
        width: 2,
        height: 2
    };

    println!("{}", rectangle.get_area());

}