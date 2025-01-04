pub mod garden;

mod my_module {
    pub struct MyStruct {
        x: i32,
    }

    pub fn new(x: i32) -> MyStruct {
        MyStruct { x }
    }
}

fn main() {
    let test = garden::veg::Asparagus;
    println!("{:?}", test);

    
    // This works because MyStruct is public:
    let s = my_module::MyStruct { x: 10 };
    
    
}
