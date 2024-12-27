fn main() {
    string_lit();
    move_copy();
    deep_copy();
    stack_copy();

}

fn string_lit() {
    let _a = "hello"; // string literal is immutable and stored on the stack
    // Requesting memory from the memory allocator at runtime
    let mut b = String::from("hello"); // string object is mutable and stored on the heap

    b.push_str(", world!"); // push_str() appends a literal to a String

    println!("{b}");

    //optionall we can call
    drop(b); // return memory to the allocator
}
// Rust takes a different path: the memory is automatically returned once 
// the variable that owns it goes out of scope.

fn move_copy(){
    let s1 = String::from("hello");
    let _s2 = s1; // s1 is moved to s2
    // println!("{s1}"); // error: value borrowed here after move
    // This is because Rust does not allow multiple variables to own the same memory location.
    // A practice used to enure memory safety, because when both variables go out of scope,
    // they will try to free the same memory location, which can cause a double free error and memory corruption.
}

fn deep_copy(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is cloned to s2, causing a duplication of the memory location on the heap
    println!("{s1}, {s2}");
}

fn stack_copy(){
    let x = 5;
    let y = x; // x is copied to y
    println!("{x}, {y}");

    // Since integers have a known size at compile time, they can be stored on the stack, unlike mutable String objects that can have a variable size.
    // Therefore, copies of the actual values are quick to make.
}