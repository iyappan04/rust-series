fn main(){

    let mut data = vec![1,4,7];

    // when we give move closure it;s says that we passing the ownership 

    let mut add = move || {
        data.push(5);
        println!("{:?}", data);
    };

    // it will end by here, can't print outside of the move closure

    // println!("{:?}", data);

}