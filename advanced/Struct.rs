// Rust is a composite datatype 
// It's group of variable's

struct User{
    id: i32,
    username: String,
    email: String,
    password: String,
    isAdmin: bool
}

impl User {

    fn register(&self){
        println!("{} {} {}", self.username, self.email, self.password);
    }

    fn check_admin(&self){
        if(self.isAdmin){
            println!("Admin User");
        }
        else{
            println!("User");
        }
    }

}

fn main(){

    let user1 = User {
        id: 1,
        username: "iyappan".to_string(),
        email: "iyappank165@gmail.com".to_string(),
        password: "User9344#".to_string(),
        isAdmin: true
    };

    user1.register();
    user1.check_admin();

    // println!("{}", user1.email);

}