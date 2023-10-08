use std::{thread, time::Duration};

pub fn threads_demo() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from new thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

}

