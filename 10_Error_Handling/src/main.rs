/*

use std::fs;


fn main() {
    //there is a fn that can error out/stop the thread , Result prevents this error by returning a Result containing the error
    let res = fs::read_to_string("../cargo.toml");

    match  res {
        Ok(content) => println!("File content: {}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    // println!("{}", read_from_file_unsafe("example.txt".to_string()));
}


fn read_from_file_unsafe(file_content: String)->String{
    let res = fs::read_to_string("example.txt");
    return  res.unwrap(); // This will panic if the file does not exist or cannot be read
    // Note: Using unwrap() is not recommended for production code as it can cause the program
}

fn read_from_file_safe(file_content: String)->Result<String, String>{
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
    // This function returns a Result, allowing the caller to handle errors gracefully
    // without panicking.
    //Another way to write this function using if let
    // if let Ok(content) = res {
    //     return Ok(content);
    // } else {
    //     return Err(format!("Error reading file: {}", res.unwrap_err()));
    // }
} */

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32); //returning an enum of founded value
        }
    }
    return None; //returning an enum of not found value
}

fn main() {
    let my_string = String::from("raman");
    let res = find_first_a(my_string);
    match res {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}
