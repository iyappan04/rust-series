
// enum is user defined variable which have mulitple value

enum Role {
    Admin,
    User,
    Manager,
    SuperAdmin,
    CustomRole(String),
    TotalUsers(u32),
}

fn main(){

    // let currentRole = Role::CustomRole(String::from("Lead"));
    let currentRole = Role::TotalUsers(5);

    match currentRole {
        Role::Admin => println!("This is Admin User"),
        Role::User => println!("This is Normal User"),
        Role::Manager => println!("This is Manager"),
        Role::SuperAdmin => println!("This is SuperAdmin"),
        Role::CustomRole(value) => println!("{}", value),
        Role::TotalUsers(value) => println!("{}", value),
    }

}