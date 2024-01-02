// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    name: String,
    color: String,
}

// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector

// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
fn display_name(name: &str) {
    println!("name: {:?}", name);
}

fn display_color(color: &str) {
    println!("color: {:?}", color);
}

fn main() {
    let people = vec![
        Person {
            age : 32,
            name : "Punpun".to_owned(),
            color : "red".to_owned(),
        },
        Person {
            age: 9,
            name: "DJ".to_owned(),
            color: "blue".to_owned(),
        },
        Person {
            age: 20,
            name: "Kan".to_owned(),
            color: "green".to_owned(),
        },
    ];

    for person in &people {
        if person.age <= 10 {
            display_name(&person.name);
            display_color(&person.color);
        }
    }

    
}
