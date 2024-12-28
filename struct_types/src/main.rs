fn main() {
    struct User{
        name: String,
        email: String,
        _active: Option<bool>,
    }

    let user1 = User{
        name: String::from("John Doe"),
        email: String::from("JDoe@email.com"),
        _active: None,
    };

    let _user2 = User{
        name: String::from("Jane Doe"),
        email: String::from("JaDoe@email.com"),
        _active: Some(true), // When setting the value need to specific Some(value_type) or None, due to the Option<> type
    };

    println!("User 1 name is {0}, email is {1}", user1.name, user1.email);
}
