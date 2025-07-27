use std::collections::HashMap;

fn main() {
    let mut vec = Vec::new(); //Initialize a new empty vector
    vec.push(1); //Add an element to the vector
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec); //Printing format for vectors, because its kinda a struct underneath so {} doesn't work, we use {:?} instead which is a debug trait
    println!("First element: {}", vec[0]); //Accessing the first element using indexing

    //Another way to initialize a vector is to use vec! macro
    // let vec2 = vec![1, 2, 3, 4, 5]; //This is a macro that initializes a vector with the given elements


    let mut users:HashMap<String,i32> = HashMap::new();

    users.insert(String::from("Ayush"), 21);
    users.insert(String::from("John"), 22);

    let first_user_age = users.get("Ayush"); //Return type is Option<&i32>, because the key might not exist in the HashMap

    match first_user_age {
        Some(age) => println!("Ayush's age is {}", age),
        None => println!("User not found"),
    }


}
//Assignment 1: Write a function that takes a vector and returns a new vector with even numbers only

//Assignment 2: Write a function that takes a vector of tuples(each tuple contains a key and a value) and returns a HashMap where the keys are unique keys from input tuples and values are vectors of all corresponding values assoiciated with each key. 