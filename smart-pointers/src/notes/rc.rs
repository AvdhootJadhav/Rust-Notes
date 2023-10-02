use std::rc::Rc;

use crate::notes::rc::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil
}

pub fn rc_demo() {

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a {}", Rc::strong_count(&a));

    let _b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b {}", Rc::strong_count(&a));

    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Count after creating c {}", Rc::strong_count(&a));
    }
    
    println!("Going out of scope {}", Rc::strong_count(&a));

}

