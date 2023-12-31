use std::{sync::{Mutex, Arc}, thread};

pub fn shared_state_demo() {
    
    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    print!("counter : {:?}", *counter.lock().unwrap());
}