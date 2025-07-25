use chrono::{Local,Utc}; //installed via `cargo add chrono`


fn main() {

    let now = Local::now();
    println!("Current time in Local: {}", now);

    let now_utc = Utc::now();
    println!("Current time in UTC: {}", now_utc);

    println!("Hello, world!");
}
