use std::{sync::mpsc, thread, time::Duration};

pub fn message_passing_demo() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let data = vec![
            String::from("Hi"),
            String::from("How are you?"),
            String::from("So how is your job going?"),
            String::from("I am good just enjoying the weekend")
        ];

        for item in data {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || {
        let data = vec![
            String::from("Hello"),
            String::from("I am good"),
            String::from("I am enjoying my job"),
            String::from("What about you?")
        ];

        for item in data {
            tx1.send(item).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });


    for item in rx {
        println!("Got: {}", item);
    }
}