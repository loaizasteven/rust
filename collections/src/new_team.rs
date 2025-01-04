use std::collections::HashMap;

pub fn initialize_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores
}