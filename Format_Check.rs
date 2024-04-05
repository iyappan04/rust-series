struct Person {
    name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Drop Instance Dying");
    }
}

fn main() {

    let _person = Person { name: String::from("iyappan") };
    let _person1 = Person { name: String::from("something") };
}
