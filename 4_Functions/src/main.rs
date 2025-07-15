fn main() {
    let a = 5;
    let b = 10;

    // Call the function to sum a and b
    let result = do_sum(a, b);

    // Print the result
    println!("The sum of {} and {} is: {}", a, b, result);
}

fn do_sum(a: i32, b: i32) -> i32 { //We need to define return type explicity , it can't be infered automatically like in TypeScript
    // a + b
    return a + b;
}