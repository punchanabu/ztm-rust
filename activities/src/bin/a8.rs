// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavors {
    Strawberry,
    Vanilla,
    Chocolate,
    Mint,
}

struct Drink {
    flavor: Flavors,
    ounces: f64,
}

fn display(drink : Drink) {
    match drink.flavor {
        Flavors::Strawberry => println!("Flavor: Strawberry"),
        Flavors::Vanilla => println!("Flavor: Vanilla"),
        Flavors::Chocolate => println!("Flavor: Chocolate"),
        Flavors::Mint => println!("Flavor: Mint"),
        _ => println!("Flavor: Unknown flavour!")
    };
    println!("ounce: {:?}", drink.ounces);
}
fn main() {
    let my_drink = Drink {
        flavor : Flavors::Vanilla,
        ounces : 32.4,
    };

    display(my_drink);
}
