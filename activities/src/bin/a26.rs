// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

fn main() {

    use chrono::prelude::*;

    let utc: DateTime<Local> = Local::now();

    println!("UTC now is: {}", utc.format("%Y-%m-%d %H:%M:%S").to_string());
    
}
