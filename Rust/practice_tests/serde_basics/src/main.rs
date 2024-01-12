use serde::{Deserialize, Serialize};
use serde_json;
use std::any::type_name;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Serialization
    let person = Person {
        name: String::from("John Doe"),
        age: 30,
    };
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);
    println!("Serialized: {}", type_name::<serialized>());

    // Deserialization
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
