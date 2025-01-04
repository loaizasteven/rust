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

fn main() {
    vec_example();
}
