use std::{io, collections::HashMap};
#[derive(Debug, Clone)]
pub struct Bill {
    name : String,
    amount : f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        return self.inner.values().collect();
    }

    fn remove(&mut self, name: &str) -> bool {
        return self.inner.remove(name).is_some();
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                return true;
            },
            None => return false
        }
    }
}
fn get_input() -> Option<String> {
    let mut buffer: String  = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again:");
    }
    let input: String = buffer.trim().to_owned();
    if &input == "" {
        return None
    } else {
        return Some(input)
    }
} 

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result <f64,_> = input.parse();

        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}
mod menu {
    use crate::{Bills, Bill, get_input, get_bill_amount};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        let name: String = match get_input() {
            Some(input) => input,
            None => return
        };
        let amount = match get_bill_amount() {
                    Some(input) => input,
            None => return
        };
        let bill: Bill = Bill {name, amount};
        bills.add(bill);
        println!("Bills added");
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove:");

        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        if bills.remove(&name) {
            println!("bill removed");
        } else {
            println!("bill not found");
        }
    }
    
    pub fn update_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill to update:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) {
            println!("updated");
        }
        else {
            println!("bill not found");
        }
    }

}
enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}

impl MainMenu {
    fn from_str(input : &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None
        }
    }
    fn show() {
        println!("");
        println!("== Welcome to Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("");
        println!("Enter Selection: ")
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();


    loop {
        MainMenu::show();
        let input: String = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break, 
        }
    }
    None
}
fn main() {
    run_program();
}