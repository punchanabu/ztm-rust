// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}


fn main() {
    let tickets = vec![
        Ticket::Backstage(150.0, "Alice".to_owned()),
        Ticket::Vip(120.0, "Bob".to_owned()),
        Ticket::Standard(80.0),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("BackStage! name: {:?}, price: {:?}", name, price)
            },
            Ticket::Vip(name, price) => {
                println!("Vip! name: {:?}, price {:?}", name, price)
            },
            Ticket::Standard(price) => {
                println!("Standard price : {:?}", price)
            },
        }
    };
}
