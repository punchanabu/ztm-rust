// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {

    // spawn threads here
    let msg_one = std::thread::spawn(move || msg_hello());
    let msg_two = std::thread::spawn(move || msg_thread());
    let msg_three = std::thread::spawn(move || msg_excited());

    // the main wait on a particular thread to finsh 
    let msg_one = msg_one.join().expect("failed to join msg1");
    let msg_two = msg_two.join().expect("failed to join msg2");
    let msg_three = msg_three.join().expect("failed to join msg3");
    
    // print the message
    println!("{}{}{}", msg_one, msg_two, msg_three);
}