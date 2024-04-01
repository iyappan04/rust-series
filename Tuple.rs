fn main(){


    let tup = (1,"iyappan", 75.4, true);

    println!("{}", tup.2);

    let ( a, b, c, d ) = tup;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

}