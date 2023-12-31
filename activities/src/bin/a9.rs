// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
fn get_cor() -> (i32, i32) {
    return (10, 8);
}




fn main() {
    // * Destructure the return value into two variables
    let (x, y) = get_cor();
    
    // * Use an if..else if..else block to determine what to print
    if y == 5 {
        println!("equal y values");
    }
    else if y > 5 {
        println!("greater y values");
    }
    else {
        println!("less than y values");
    }
}
