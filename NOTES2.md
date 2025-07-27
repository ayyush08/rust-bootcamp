# Collections

Rust's standard library includes a number of very useful data structures called collections. 
Most other data types represent one specificy value, but collections can hold multiple values.
The data these collections point to is stored on the heap, which allows for dynamic sizing.

## 1. Vectors
Vectors are the most commonly used collection type in Rust. They are similar to arrays but can grow and shrink in size. Puts all values next to each other in memory, which allows for fast access.

```rust
fn main() {
    let mut vec = Vec::new(); //Creating a new vector, this is an empty vector
    vec.push(1); //Adding an element to the vector
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
 
    println!("Vector: {:?}", vec); //Printing the vector
    println!("First element: {}", vec[0]); //Accessing the first element using indexing
    //Another way to initialize a vector is to use vec! macro
    let vec2 = vec![1, 2, 3, 4, 5]; //This is a macro that initializes a vector with the given elements

}
```

### Defining type of vector as a generic type

Explicitly defining the type of the vector can help with performance and clarity.
But it is not always necessary, as Rust can often infer the type from context.

```rust
fn main(){
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; //Defining a vector of i32 type
    println!("Numbers: {:?}", numbers);
    let strings: Vec<String> = vec!["Hello".to_string(), "World".to_string()]; //Defining a vector of String type

    //IF NOT EXPLICITLY DEFINED, RUST WILL INFER THE TYPE FROM THE CONTEXT AS IT DOES WITH ALL VARIABLES
}
```

## 2. HashMaps
Stores a key value pair in rust.
Similar to objects in JS
Dictionaries in Python
HashMaps in Java

Methods:
- `insert(key, value)`: Adds a key-value pair to the map.
- `get(&key)`: Retrieves a reference to the value associated with the key.
- `remove(&key)`: Removes the key-value pair from the map.
- `clear()`: Removes all key-value pairs from the map.


```rust
use std::collections::HashMap;  
fn main(){
    let mut users: HashMap<String, i32> = HashMap::new(); //Creating a new HashMap with String keys and i32 values
    users.insert(String::from("Alice"), 30); //Inserting a key-value pair into the HashMap
    users.insert(String::from("Bob"), 25);

    let user1 = users.get("Alice"); //Retrieving a value by key, returns an Option<&i32>
}