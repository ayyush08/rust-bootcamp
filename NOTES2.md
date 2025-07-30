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

```
# Iterators

The iterator patter allows you to perform some task on a sequence of items in turn. An iterator is responsible for logic of iterating over each item and determing when the sequence has finished. When you use iterators, you don't have to reimplement that logic yourself.

In Rust, iterators are ```lazy```, meaning they have no effect until you call methods that consume the iterator to use it up.

For example, the code in creates an iterator over the items in the vector `v1` by calling the `iter` method defined on `Vec<T>`. This code by itself does not do anything useful


```rust
let v1 = vec![1, 2, 3];
let mut iter = v1.iter(); // Create an iterator over the vector(Not using just defined)
```


## Type of Iterators

### 1. Using for loop

```rust
let nums = vec![1, 2, 3, 4, 5];
for value in num{
    println!("Value: {}", value); //This will print each value in the vector
}
```

### 2.  After creating an `iterator`

```rust
let nums = vec![1, 2, 3, 4, 5];
let iter = nums.iter(); //Creating an iterator over the vector
for value in iter {
    println!("Value: {}", value); 
}
```

The `iter` method in Rust provides a way to iterator over the elements of a collection by borrowing them and not consuming them. This means you can still use the original collection after iterating over it. 

You can't mutate these variables since we have an immutable reference to the internal elements

### 3. IterMut (Mutable Iterators)

```rust
let mut nums = vec![1, 2, 3, 4, 5];
let mut it = nums.iter_mut(); //Creating a mutable iterator

for val in it{
    *val += 1; //Mutating the value by dereferencing it
}
println!("Updated nums: {:?}", nums); //Printing the updated vector
```


### 4. Iterator using `.next`

```rust
fn main(){
    let nums = vec![1, 2, 3]
    let mut iter = nums.iter(); 
    //iter.next() returns an Option<&i32>, which is the next value in the iterator
    while let Some(val) = iter.next() {
        println!("Value: {}", val); //This will print each value in the vector
    }
}
```

### 5. IntoIter

```rust
fn main(){
    let nums = vec![1, 2, 3];
    let iter = nums.into_iter(); 

    for val in iter {
        println!("Value: {}", val); 
    }
    // nums is no longer accessible here, as ownership has been moved to the iterator
    // println!("{:?}", nums); // This will cause a compile-time error
}
```
The `into_iter` trait is used to convert a collection into an iterator that takes ownership of the collection. 

Useful when:
1. You no longer need the original collection.
2. When you need to squeeze performance benefits by transferring ownership (avoiding references)

NOTE:
The for syntax when applied direclty on the collection uses `into_iter` by default, which means it consumes the collection.

```rust
fn main(){
    let nums = vec![1, 2, 3];
    for val in nums { // This uses into_iter implicitly
        println!("Value: {}", val); 
    }
    // nums is no longer accessible here, as ownership has been moved to the iterator
    // println!("{:?}", nums); // This will cause a compile-time error
}
```

## Which to use?

- Use `iter` , if you want immutable references to the inner variables and don't want to transfer ownership.

- Use `iter_mut` , if you want mutable references to the inner variables and don't want to transfer ownership.

- Use `into_iter` , if you want to move the variable into the iterator and don't want to use it afterwards.


## Consuming Adapters

Methods that call next are called consuming adapters.  because calling them uses up the iterator, meaning you can't use it again after calling them.

```rust
let nums = vec![1, 2, 3];
let it = nums.iter();

int sum: i32 = it.sum(); // This consumes the iterator and calculates the sum of the elements
println!("Sum: {}", sum); // This will print the sum of the elements in the vector
// println!("{:?}", it); // This will cause a compile-time error, as the iterator has been consumed
```

## Iterator Adapters

Iterator adapters are methods defined on the iterator trait that don't consume the iterator. Instead, they produce different iterators by changin some aspect of original iterator.

```rust
fn main(){
    let v1 = vec![1, 2, 3, 4, 5];
    let iter = v1.iter();

    let iter2 = iter.map(|x| x * 2); // This creates a new iterator that doubles each value in the original iterator

    for x in iter2{
        println!("Value: {}", x); // This will print each value in the new iterator
    }

}
```
Similar to the `map` , `filter` is another common iterator adapter that allows you to filter elements based on a condition.

```rust
fn main(){
    let v1 = vec![1, 2, 3, 4, 5];
    let iter = v1.iter();

    let iter2 = iter.filter(|x| *x % 2 == 0); // This creates a new iterator that only includes even numbers

    for x in iter2{
        println!("Even Value: {}", x); // This will print each even value in the vector
    }
}
```

**Note:**  To convert a iterator back into a collection, you can use the `collect` method. This method consumes the iterator and produces a collection of the specified type.
```rust
fn main(){
    let v1 = vec![1, 2, 3, 4, 5];
    let iter = v1.iter();

    let doubled: Vec<i32> = iter.map(|x| x * 2).collect(); // This creates a new vector with doubled values

    println!("Doubled Values: {:?}", doubled); // This will print the new vector with doubled values
}
```


# Strings vs Slices

The ```String``` type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to "strings,"  in Rust, they might be referring to either the `String` type or the string slice type `&str`, not just one of those types. 

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership. 


### 1. Creating a String

```rust
fn main() {
    let s = String::from("Ayush ");
    println!("String: {}", s); 
}
```

### 2. Mutating a String

```rust
fn main() {
    let mut s = String::from("Ayush ");
    s.push_str("Kumar"); //Appending a string to the existing string
    println!("Mutated String: {}", s); 
}
```

### 3. Deleting a String

```rust
fn main() {
    let mut s = String::from("Ayush Kumar");
    s = s.replace_range(6..s.end(),""); //Removing a part of the string
    println!("Deleted String: {}", s); //This will print "Ayush "
}
```

- ## String Literals
String literals are immutable references to a string slice. They are stored in the read-only memory of the program and cannot be changed. 

```rust
fn main(){
    let word = "Hello World"; //This is a string literal, which is also a &str type, but it points directly to and address in binary
    println!("String Literal: {}", word); 
}
