pub mod garden;

#[allow(dead_code)]
mod my_module {
    pub struct MyStruct {
        pub x: i32
    }

    pub fn _new(x: i32) -> MyStruct {
        MyStruct { x }
    }
}

fn main() {
    let test = garden::veg::Asparagus;
    println!("{:?}", test);

    
    // This works because MyStruct is public:
    let _s = my_module::MyStruct { x: 10 };
    
    
}
