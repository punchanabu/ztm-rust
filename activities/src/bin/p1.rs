// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed. --- done
//   - I want to view existing bills. -- done
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}


#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}
impl Bill {
    fn new(name: &str, amount: f64) -> Bill {
        Bill {
            name: name.to_owned(),
            amount,
        }
    }
}

fn add_bill( bills : &mut Vec<Bill>) -> io::Result<()> {
    
    println!("Please enter the name of the bill");
    let name = get_input()?;
    if name.to_lowercase() == "back" {
        println!("Operation canceled.");
        return Ok(());
    }
    println!("Please enter the amount of bill");
    let amount_str = get_input()?;
    let amount : f64 = match amount_str.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount");
            return Ok(());
        }
    };
    
    bills.push(Bill::new(&name, amount));
    println!("Adding bills successful!, Your bills is now {:?}", bills);

    Ok(())
}

fn view_bills(bills: &Vec<Bill>) {
    if bills.is_empty() {
        println!("You have no bills");
        return;
    }
    println!("Your bills are:");
    let mut total = 0.0;
    for bill in bills.iter() {
        println!("{}: ${}", bill.name, bill.amount);
        total += bill.amount;
    }
    println!("Total: ${}", total);
}

fn remove_bill(bills: &mut Vec<Bill>) -> io::Result<()> {
    println!("Please enter the name of the bill to remove");
    let name = get_input()?;
    if name.to_lowercase() == "back" {
        println!("Operation canceled.");
        return Ok(());
    }
    let index = get_index(&bills, &name);
    match index {
        Some(i) => {
            bills.remove(i);
            println!("Bill removed successfully!");
            Ok(())
        },
        None => {
            println!("Bill not found");
            Ok(())
        },
    }
}

fn edit_bills(bills: &mut Vec<Bill>) -> io::Result<()> {
    println!("Please enter the bill name you want to edit: ");
    let name = get_input()?;
    if name.to_lowercase() == "back" {
        println!("Operation canceled.");
        return Ok(());
    }
    let idx = get_index(&bills, &name);
    
    match idx {
        Some(i) => {
            println!("Please enter the new amount:");
            let amount_str = get_input()?;
            match amount_str.parse() {
                Ok(num) => {
                    bills[i].amount = num;
                    println!("Bill edited successfully!");
                    Ok(())
                },
                Err(_) => {
                    println!("Invalid amount");
                    Ok(())
                }
            }

        },
        None => {
            println!("Bill not found");
            Ok(())
        }
    }
}

fn get_index(bills : &Vec<Bill>, name : &str) -> Option<usize> {
    for (i,bill) in bills.iter().enumerate() {
        if bill.name == name {
            return Some(i);
        }
    }
    return None
}
fn main() {
    let mut bills: Vec<Bill> = Vec::new();
    loop {
        println!("Welcome to the bill manager!");
        println!("Please choose an option:");
        println!("1. Add a bill");
        println!("2. View bills");
        println!("3. Remove a bill");
        println!("4. Edit a bill");
        println!("5. Exit");
        let mut buffer = String::new();
        let buffer_result = io::stdin().read_line(&mut buffer);
        if let Err(e) = buffer_result {
            println!("Error: {}", e);
            continue;
        }
        let choice = buffer.trim().to_owned();
        match choice.as_str() {
        
            "1" => {
                if let Err(e) = add_bill(&mut bills) {
                    println!("Error: {}", e);
                }
            },
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills).expect("Error removing bill"),
            "4" => {
                if let Err(e) = edit_bills(&mut bills) {
                    println!("Error: {}", e);
                }
            },
            "5" => break,
            _ => {
                println!("Invalid choice")
            },
        }
    }
}
    


