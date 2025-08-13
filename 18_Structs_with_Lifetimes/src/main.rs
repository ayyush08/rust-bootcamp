// struct User {
//     name: &str,
//}//How do we know, how long is user valid for? Shouldn't it be tied to the lifetime of name? As if name goes out of scope then user should also go out of scope.

struct User<'a> {
    name: &'a str, // Here we are saying that the lifetime of name is tied to the lifetime of User
}

fn main() {
    let name = String::from("Alice");
    let user = User { name: &name };

    println!("User name: {}", user.name);
}
