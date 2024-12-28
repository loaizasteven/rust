#[derive(Debug)] // This is a derive attribute that allows us to print the struct using the {:?} formatter
struct User {
    name: String,
    email: String,
    _active: Option<bool>,
}

fn main() {
    // Structs are similar to tuples, but with named fields
    fn build_user(name: String, email: String) -> User {
        User {
            name, // With field init shorthand syntax, we can use the variable name as the field name
            email,
            _active: Some(true), // When setting the value need to specific Some(value_type) or None, due to the Option<> type
        }
    }

    let user1 = User {
        name: String::from("John Doe"),
        email: String::from("JDoe@email.com"),
        _active: None,
    };

    let mut user2 = build_user(String::from("Jane Doe"), String::from("JDoe@email.com"));
    // Duplicate user2 using struct update syntax
    let _user2_updated = User {
        _active: Some(false),
        ..user2 // specify remaining fields to be moved/copied from user2
    };
    // note because we changed the value of user2.email, we can't use user2.email anymore since its ownership has been moved to user2_updated
    // println!("User 2 email is {0}", user2.email); // This will throw an error "value borrowed here after move"
    user2.email = String::from("JaDoe@email.com");
    println!("User 1 name is {0}, email is {1}", user1.name, user1.email);
    println!("User 1 Object is: {user1:#?}"); // pretty print with #?
}
