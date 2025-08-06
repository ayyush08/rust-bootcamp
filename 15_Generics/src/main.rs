fn main(){
    let bigger = largest(1,2);
    let bigger_char = largest('a', 'b');
    println!("Bigger i32: {}", bigger);
    println!("Bigger char: {}", bigger_char);
}
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}