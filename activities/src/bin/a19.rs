// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use  std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    store.insert("Chairs", 5);
    store.insert("Beds", 3);
    store.insert("Tables", 2);
    store.insert("Couches", 0);

    let mut sum = 0;

    for (key,val) in store.iter() {
        
        let cnt = if val == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", val)
        };
        println!("item= {:?}: stock = {:?}", key, cnt);
        
        sum = sum + val;
    }

    println!("Total number of items in stock: {:?}", sum);
    
}
