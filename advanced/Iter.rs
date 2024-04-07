fn main(){

    let numbers = vec![1,2,3,5,3,7];

    // as it's normal we are not giving any ownership and all 
    // we can print the vec outer of the for loop.

    for number in numbers.iter() {
        println!("{}", number);
    }


    // here we are passing the ownership

    for number in numbers.into_iter() {
        println!("{}", number);
    }

    println!("{:?}", numbers); // it will print the error, bcz of ownership gone

     // here we are passing the ownership

    let mut nums = vec![1,2,3,5,3,7]; // it;s only for mutable reference

    for number in nums.iter_mut() {
        *number*=2;
    }

    println!("{:?}", nums);


}