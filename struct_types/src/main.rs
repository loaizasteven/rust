fn main() {
    struct User{
        name: String,
        email: String,
        _active: Option<bool>,
    }

    fn build_user(name: String, email: String) -> User{
        User{
            name, // With field init shorthand syntax, we can use the variable name as the field name
            email,
            _active: Some(true), // When setting the value need to specific Some(value_type) or None, due to the Option<> type
        }
    }

    let user1 = User{
        name: String::from("John Doe"),
        email: String::from("JDoe@email.com"),
        _active: None,
    };

    let mut user2 = build_user(String::from("Jane Doe"), String::from("JDoe@email.com"));
    // Duplicate user2 using struct update syntax
    let _user2_updated = User{
        _active: Some(false),
        ..user2 // specify remaining fields to be copied from user2
    };

    user2.email = String::from("JaDoe@email.com");
    println!("User 1 name is {0}, email is {1}", user1.name, user1.email);
}