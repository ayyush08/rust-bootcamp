fn main() {
    let s1 = String::from("hello world");
    println!("{}", s1); // This works, s1 is valid here(not yet moved)
    let s2 = s1;
    // println!("{}", s1); // error: value borrowed here after move
    println!("{}", s2); // This would work, but s1 is no longer valid
}
