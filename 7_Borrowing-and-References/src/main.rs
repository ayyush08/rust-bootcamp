fn main() {
    // let s1 = String::from("Hello");
    // let s2 = &s1;

    // println!("{}", s2);
    // println!("{}", s1);    // This is valid, The first pointer wasn't invalidated
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    update_str(s2);
    // update_str(&mut s1); //Error: You can  only have one mutable reference to a piece of data in a particular scope
    // println!("{}",s1);
    println!("{}",s2);

}

/*
fn update_str(s: &String){
    s.push_str("World"); //cannot borrow `*s` as mutable, as it is behind a `&` reference `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
*/

fn update_str(s: &mut String) { //Borrowing mutably
    s.push_str(" World"); // This is valid, `s` is a mutable reference
}