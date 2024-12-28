#![allow(dead_code)] // allow unsured code without warning

// create a struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn debug(&self) {
        println!("Debugging the struct Rectangle: {self:#?}");
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Assocaited function that is not a method, because it does not have a self parameter in the signature
    // Self is an alias for the struct after the impl keyword
    // -----
    // " Associated functions that aren’t methods are often used for constructors that will return a new 
    // " instance of the struct. 
    // " These are often called new, but new isn’t a special name and isn’t built into the language." - Rust Book
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let _area_val = rect.area();
    rect.debug();

    let sq = Rectangle::square(3); // call assocaited function
    println!("Square: {sq:#?}");
}
