// struct User{
//     name: String,
//     age: u32,
//     active: bool,
// }
// fn main() {
//     let name = String::from("Ayush");

//     let user = User{
//         name:name,
//         age: 20,
//         active: true,
//     };
//     println!("{} is {} years old.", user.name, user.age);
// }

struct Rect {
    width: u32,
    height: u32,
}

impl Rect { //implementation of struct Rect
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    print!("The area of the rectangle is {}", rect.area());
    println!(" and the perimeter is {}", rect.perimeter());
}
