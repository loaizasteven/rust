fn main() {
    vec_example();
    vec_ownership();
    vec_iter();
    vec_datatypes();
}

pub fn vec_example() {
    // There are two ways to reference a value stored in a vector: via indexing or by using the get method. 
    // In the following examples, we’ve annotated the types of the values that are returned from these 
    // functions for extra clarity.
    
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

pub fn vec_ownership() {
    // panics because we have an immutable reference to the first element of the vector
    // and we try to push a new element to the vector
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    let first = &v[0];

    // v.push(7);
    println!("The first element is: {first}");

}

pub fn vec_iter() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // dereference i and add 50 to the value
    }
    println!("{:?}", v);
}   

pub fn vec_datatypes() {
    // We can define a vector to hold different types of data using an enum

    #[derive(Debug)] 
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}