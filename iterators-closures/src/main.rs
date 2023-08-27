use iterators_closures::tuts::iterators::iterator_demo;

fn main() {
    // let inventory = Inventory {
    //     shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
    // };

    // let user_pref = None;

    // let giveaway = inventory.giveaway(user_pref);

    // println!("The user having preference {:?} gets shirt with color {:?}", user_pref, giveaway);

    // let mut list = vec![1, 2, 20, 34, 29];

    // println!("List: {:?}", list);

    // if there is one mutable borrow then no immutable borrow allowed

    // thread::spawn(move || println!("Inside closure: {:?}", list))
    //     .join()
    //     .unwrap()
    
    iterator_demo()

}
