use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type SharedSignData = Arc<Mutex<String>>;

struct DigitalSignBoard {
    display: SharedSignData,
}

impl DigitalSignBoard {
    fn update(&self) {
        let data = self.display
            .lock();
        println!("sign data = {}", data);
    }
}

fn spawn_display_thread(display_data : SharedSignData) -> thread::JoinHandle<()> {
    thread::spawn(|| {
        let board = DigitalSignBoard {
            display: display_data,
        };
        loop {
            board.update();
            thread::sleep(Duration::from_secs(1));
        }

    })
}

fn change_data(display_data :   SharedSignData, new_data : &str) {
    let mut data = display_data.lock();
    *data = new_data.to_owned();
    println!("----updated data to {}", new_data);
}


fn main() {
    let display_data = Arc::new(Mutex::new(String::from("initial")));
    spawn_display_thread(display_data.clone());
    
    thread::sleep(Duration::from_secs(5));
    change_data(display_data.clone(), "message-3");

    thread::sleep(Duration::from_secs(5));
    change_data(display_data.clone(), "message1");
}



