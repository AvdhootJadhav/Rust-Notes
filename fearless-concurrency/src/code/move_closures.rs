use std::thread;

pub fn move_and_closures() {

    let arr = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("arr vector looks like this {:?}", arr);
    });


    handle.join().unwrap();
    
}