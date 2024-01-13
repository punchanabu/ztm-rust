// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing
#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

struct ShirtColor(Color);
struct PantsColor(Color);
struct ShoesColor(Color);


impl ShirtColor {
    fn new(color: Color) -> Self {
        ShirtColor(color)
    }
}

impl PantsColor {
    fn new(color: Color) -> Self {
        PantsColor(color)
    }
}


impl ShoesColor {
    fn new(color: Color) -> Self {
        ShoesColor(color)
    }
}

fn shirt(color: ShirtColor) {
    println!("Shirt color: {:?}", color.0);
}

fn pants(color: PantsColor) {
    println!("Pants color: {:?}", color.0);
}

fn shoes(color: ShoesColor) {
    println!("Shoes color: {:?}", color.0);
}


fn main() {
    let shirt_color = ShirtColor::new(Color::Black);
    let pants_color = PantsColor::new(Color::Blue);
    let shoes_color = ShoesColor::new(Color::Red);

    shirt(shirt_color);
    pants(pants_color);
    shoes(shoes_color);
}



    