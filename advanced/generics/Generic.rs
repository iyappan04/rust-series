// use custom datatype's, and reduce code redundancy 

struct Person<T: Car> {
    name: String,
    car: T
}

impl <T:Car> Person<T> {
    fn print_info(&self) {
        println!("Name = {}", self.name);
        self.car.print_specification();
    }
}

trait Car {
    fn print_specification(&self);
}

struct BMW {
    price: i64,
}

impl Car for BMW {
    fn print_specification(&self){
        println!("BMW");
        println!("BMW price is {}", self.price)
    }
}

struct AUDI {
    has: bool,
}

impl Car for AUDI {
    fn print_specification(&self){
        println!("AUDI");
        println!("AUDI has {}", self.has)
    }
}



fn main(){ 

    let bmw1 = BMW { price: 494653924 };

    let audi = AUDI { has : false };

    let person1 = Person {
        name: String::from("iyappan"),
        car: audi
    };

    person1.print_info();

}
