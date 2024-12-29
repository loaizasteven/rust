#![allow(unused)]
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) -> i32 {
            // method body would be defined here
            match self {
                Message::Quit => 0,
                Message::Write(a) => {
                    println!("Write: {a}"); // "a" is an alias for the value of the String, but also referred to as a "pattern that binds to the value"
                    1
                }
                _ => { // catch-all "_" pattern let rust know that we want to match all other cases but not use the value, otherwise we would have to use a variable name such as "other"
                    println!("This is a catch-all");
                    return 2
                }
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    let response = m.call();
    println!("{response:#?}");
}