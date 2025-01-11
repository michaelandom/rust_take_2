#[derive(Debug)]
struct User {
    username: String,
    email: String,
    is_active: bool,
    login_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("wew@yopmail.com"),
        username: String::from("wew@yopmail.com"),
        is_active: true,
        login_count: 1,
    };

    println!("user 1 {:#?}", user1);
    println!("user 1 email {} ", user1.email);

    let mut user2 = create_user(
        String::from("asjda@yopmail.com"),
        String::from("asjda@yopmail.com"),
    );

    println!("user 2 {:#?}", user2);
    user2.email = String::from("user2@yopmail.com");

    println!("user 2 email {} ", user2.email);


    let user3 = User {
        email: String::from("user3@yopmail.com"),
        username: String::from("user3@yopmail.com"),
        ..user1
    };

     println!("user 3 {:#?}", user3);

}

fn create_user(email: String, username: String) -> User {
    User {
        email,
        username,
        is_active: true,
        login_count: 1,
    }
}
