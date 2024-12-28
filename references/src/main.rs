fn main() {
    let mut s1 = String::from("True");

    mutable_reference(&mut s1);
    reference(&s1);
    println!("Value of s1 is still valid: {s1}"); // s1 is still valid here

    let word = "Hello World";
    let index = first_word(&word.to_string());
    println!("The value of the first word is: {index}");
}

fn reference(s: &String){
    println!("Value of s references s1 {s}"); // 
}

fn mutable_reference(s2: &mut String){
    s2.push_str("_mutable");
    println!("Value of s2 is mutable reference of s, {s2}") // s2 chnages the value of s from the main() scope 
}

fn first_word(s: &String) -> String {
    let bytes = s.as_bytes(); // elements of string as bytes e.g. "Hello" -> [72, 101, 108, 108, 111] for signed ASCII values 

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is byte literal for space otherwise use signed ASCII value 32 
            return s[..i].to_string()
        }
    }

    return s[..].to_string()
}
