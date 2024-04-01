fn main(){


    // Array are fixed. uses stack

    let mut val = [1,2,6,2,9];

    println!("{}", val[4]);

    for i in val.iter() {

        if i%2 == 0 {
            println!("{}", i);
        }

      //  println!("{}", i);
    }

    println!("{:?}", val);



    for m in 0..val.len() {
        // println!("m is : {}", val[m]);
        for n in 0..val.len() {
            // println!("n is : {}", val[n]);
            if val[m] < val[n] {
                let temp = val[m];
                val[m] = val[n];
                val[n] = temp;
            }
        }
    }

    println!("{:?}", val);

}