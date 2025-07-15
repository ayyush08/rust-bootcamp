fn main() {
    let x = 4;
    println!("Value of signed integer x: {}", x);
    // let x: u32 = -9;// This will cause a compile-time error because `-9` is not a valid value for an unsigned integer.
    let y: u32 = 9; // This is a valid unsigned integer
    println!("\nValue of unsigned integer y: {}", y);
    let z = 3.146565; // This is a valid 32-bit floating point number
    println!("\nValue of floating point z: {}", z);


    // let mut a: i8 = 10; //In rust, to all variables are immutable by default, so we need to use `mut` keyword to make it mutable.

    // for i in 0..1000{
    //     a=a+100;
    // } - error "Attempt to add with overflow" means that the value of `a` exceeded the maximum value for `i8`, which is 127. This is because `i8` can only hold values from -128 to 127.

    //Boolean

    let is_male = false;
    let is_above_18 = true;

    if is_male {
        println!("User is male");
    }
    else{
        println!("User is not male");
    }

    if is_male && is_above_18 {
        println!("User is male and above 18");
    }


    // let mut ax = "nakdakd";

    // for i in 0..1000{
    //     ax=ax+"kknfkd"; // Doing this makes the string longer and longer, which is not a good practice in Rust and makes the code inefficient and slow.
    // }


    let greeting = String::from("Hello Nigga");
    println!("\nGreeting: {}", greeting);

    // println!("{}",greeting[0]); //This is not the right way in rust to access a character in a string. 

    // let char1 = greeting.chars().nth(0); //Notice the type of `char1` is `Option<char>` and not <char>, which means it can be `Some(char)` or `None`. Meaning how can we know if it is a character or not? It can be `None` if the index is out of bounds. So we need to handle this case.

    //We can match the `Option<char>` to check if it is `Some(char)` or `None`:
    match greeting.chars().nth(0) {
        Some(c) => println!("First character: {}", c),
        None => println!("No character at this index"),
        
    }

    //We can also use `unwrap()` to get the character, but this will panic if the index is out of bounds or if the string is empty:

    println!("First character using unwrap: {}", greeting.chars().nth(0).unwrap());





    // print!("{}",greeting.chars().nth(1000)); - This will cause a runtime error because the index is out of bounds for the string. Rust strings are UTF-8 encoded, and accessing an index that is out of bounds will panic at runtime.
}