fn tuples() {
    // fixed length, immutable
    let x: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring the tuple with pattern matching
    let five_hundred = x.0;

    // Unused variables will raise a warning in Rust, prefixing the variable name with an underscore will suppress the warning
    let _six_point_four = x.1;
    let _one = x.2;

    println!("The value of five_hundred is: {}", five_hundred);
}

fn array() {
    // fixed length, immutable, same data type
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of my_array is: {:?}", my_array);

    // Repeating the same value in an array
    let repeaiting = [3;5];
    println!("The value of repeating is: {:?}", repeaiting);
}

fn main() {
    let thousand = 0b11111111;
    println!("The value of thousand is: {}", thousand);

    let a: u8 = 57u8;
    println!("The value of type suffix is: {}", a);

    let c = 1_500_000;
    println!("The value of decimal literal is: {}", c);

    // Calling the tuples function
    tuples();
    // Calling the array function
    array();
}
