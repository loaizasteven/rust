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
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let _area_val = rect.area();
    rect.debug()
}
