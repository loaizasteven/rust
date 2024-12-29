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
                Message::Move { x, y } => {
                    println!("Move to x: {x}, y: {y}");
                    1
                }
                Message::Write(a) => {
                    println!("Write: {a}");
                    2
                }
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to r: {r}, g: {g}, b: {b}");
                    3
                }
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    let response = m.call();
    println!("{response:#?}");
}