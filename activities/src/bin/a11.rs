// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct Grocery_item {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(item : &Grocery_item) {
    println!("quantity: {:?}", item.quantity);
}

fn display_id(item: &Grocery_item) {
    println!("id: {:?}", item.id_number);
}

fn main() {

    let item = Grocery_item {
        quantity: 20,
        id_number : 32,
    };
    
    display_quantity(&item);
    display_id(&item);

    
}
