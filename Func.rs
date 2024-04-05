fn main(){
    println!("{}", factorial(5)); 
    println!("{}", fib(7));
}

fn factorial(n: i32) -> i32 {
    if n==1 {
        return 1;
    }
    else {
        return n*factorial(n-1);
    }
}


// 0 = 0 (0)
// 0+1 = 1 (1)
// 0+1 = 1 (2)
// 1+2 = 2 (3)
// 1+2 = 3 (4)
// 2+3 = 5 (5)
// 5+3 = 8 (6)
// 8+5 = 13 (7)

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    else if n==1 {
        return 1;
    }
    else{
        return fib(n-1)+fib(n-2);
    }
}