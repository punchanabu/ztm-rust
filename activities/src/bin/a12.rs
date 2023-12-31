// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:

// * Use an enum for the box color

enum Color {
    Red,
    Brown,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width {:?}", self.width);
        println!("height {:?}", self.height);
        println!("depth {:?}", self.depth);
    }
}
// * Use a struct to encapsulate the box characteristics
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl Box {
    // * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let d = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let package = Box::new(10.1, Color::Red, d);

    package.print();
}
