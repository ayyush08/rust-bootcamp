pub trait Summary {
    //similar to abstract class. a class whose function we are defining here
    fn summarize(&self) -> String {
        return String::from("No summary available");
    }
}
//Trait is like a blueprint that other structs may follow.

pub trait  Fix {
    fn fix(&self) -> String {
        return String::from("No fix available");
    }
}

struct User {
    name: String,
    age: u32,
}

struct DefaultUser {}

impl Summary for User {
    //impl is used to implement the trait for a struct
    fn summarize(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}
impl Fix for User {
    fn fix(&self) -> String {
        format!("Fixing user: {}", self.name)
    }
}

impl Summary for DefaultUser {}

fn main() {
    let user = User {
        name: String::from("Curator"),
        age: 30,
    };
    let default_user = DefaultUser {};

    println!("{}", user.summarize()); //using the summarize function defined in the trait
    println!("{}", default_user.summarize());
    notify(&user);
    notify_fix(user);
    // notify(default_user);
}

fn notify(item: &impl Summary) {//now we know that anything that uses this trait will have a summarize function
    println!("New notification: {}", item.summarize());
}

fn notify_fix<T: Summary + Fix>(item: T) { //Trait bounds
    println!("New notification: {}", item.summarize());
    println!("Fix available: {}", item.fix());
}