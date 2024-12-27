// function with a parameter, returns unit type (), similar to None.
fn my_function(x: u32) -> () {
    println!("The value of x is: {}", x)
}

fn main() {
    my_function(5);
}
