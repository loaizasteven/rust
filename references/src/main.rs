fn main() {
    let mut s1 = String::from("True");

    mutable_reference(&mut s1);
    reference(&s1);
    println!("Value of s1 is still valid: {s1}"); // s1 is still valid here
}

fn reference(s: &String){
    println!("Value of s references s1 {s}"); // 
}

fn mutable_reference(s2: &mut String){
    s2.push_str("_mutable");
    println!("Value of s2 is mutable reference of s, {s2}") // s2 chnages the value of s from the main() scope 
}
