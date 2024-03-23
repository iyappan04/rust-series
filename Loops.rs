fn main(){

    // for loop 
    for i in 1..11{
        println!("{} * {} = {}",i,2,i*2);
    }

    // while loop 
    // let mut i = 0;
    // while i<=50 {
    //     print!("{} ", i);
    //     i += 5;
    // }

    // infinite loop
    let mut n = 0;

    loop {
        print!("{} ", n);
        if n == 30 {
            break;
        }
        n += 5;
    }

}