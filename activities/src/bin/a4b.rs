// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let n = 2;
    match n {
        2 => println!("This is my favorites"),
        3 => println!("hello"),
        _ => println!("I'm so bored"),
    }
}
