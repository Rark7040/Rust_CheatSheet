use std::thread;
use std::sync::{Arc, Mutex};

pub fn _thread(){
    let data = Arc::new(Mutex::new(vec![10, 20, 30]));
    {
        let data_ref = data.clone();
        let handler = thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data.push(40);
            data.push(50);
        });
        let _ = handler.join();
    }
    println!("{:?}", data);
}