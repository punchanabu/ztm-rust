// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn display_first_name(name: &str) {
    println!("{}", name);
}

fn display_last_name(name: &str) {
    println!("{}", name);
}

fn main() {
    display_first_name("Sirawit");
    display_last_name("Chanaburanasak");
}
