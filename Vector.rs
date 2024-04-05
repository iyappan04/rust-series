fn main(){
    // Vector is a Data 
    // Grow and shrink at runtime. uses Heap memory. it's not fixed size

    let mut vec = vec![1,2,5,6];

    println!("{:?}", vec);

    vec.reverse();
    // vec.clear();
    println!("{:?}", vec);
    println!("{}", vec.len());

    let mut newvec: Vec<i32> = Vec::new();
    
    newvec.push(19);

    if newvec.contains(&19) {
        println!("Contains 19");
    }

    newvec.remove(0);

    println!("{:?}", newvec);

}