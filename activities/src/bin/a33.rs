// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn longest<'a>(lhs_string: &'a str, rhs_string: &'a str) -> &'a str {
    
    if lhs_string.len() >= rhs_string.len() {
        lhs_string
    } else {
        rhs_string
    }
    
}

fn main() {
    let short = "hello";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}
