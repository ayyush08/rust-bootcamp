// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }

// fn main() {
//     let my_direction = Direction::North;
//     move_around(my_direction);
// }

// fn move_around(direction: Direction) {
//     // implements logic to move a character around
// }

// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // if(shape == Shape::Circle((radius))){
    //     return  3.14 * radius * radius; // Area of circle
    // }
    //Better way to use Pattern Matching
    // match shape{ //Why no 'return' keyword? Because this is an expression, not a statement
    //     Shape::Circle(radius) => 3.14 * radius * radius, // Area of circle
    //     Shape::Square(side) => side * side, // Area of square
    //     Shape::Rectangle(width, height) => width * height, // Area of rectangle
    // }
    //2nd way
    let ans = match shape {
        Shape::Circle(radius) => 3.14 * radius * radius, // Area of circle
        Shape::Square(side) => side * side, // Area of square
        Shape::Rectangle(width, height) => width * height, // Area of rectangle
        
    };
    return ans;
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);


    println!("Area of Circle: {}", calculate_area(circle));
    println!("Area of Square: {}", calculate_area(square));
    println!("Area of Rectangle: {}", calculate_area(rectangle));
    
}