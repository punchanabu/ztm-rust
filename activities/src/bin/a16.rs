// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

/// Add a comment to the struct
struct Student {
    name: String,
    locker: Option<i32>,
}

/// Add a comment to the main function
fn main() {
    /// Add a comment to the student variable
    let student = Student {
        name: "Punpun".to_owned(),
        locker: Some(10),
    };
    /// Add a comment to the match expression
    match student.locker {
        Some(locker) => println!("locker: {:?}", locker),
        None => println!("No locker assigned"),
    }
}
