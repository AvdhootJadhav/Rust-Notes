use std::{rc::Rc, cell::RefCell};
use crate::notes::refcell::List::{Cons, Nil};

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a,T:Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize
}

impl <'a,T> LimitTracker<'a,T> where T:Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a,T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64/self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("Your quota is over")
        }
        else if percentage >= 0.75 {
            self.messenger.send("You have used 75% of your quota");
        }
        else if percentage >= 0.9 {
            self.messenger.send("You have used 90% of your quota");
        }
    }
}

#[cfg(test)]
mod tests{
    use std::cell::{RefCell, Ref};

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut second_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(msg));
            second_borrow.push(String::from(msg))
        }
    }

    #[test]
    fn it_sends_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(75);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

}

#[derive(Debug)]
enum List{
    Cons (Rc<RefCell<i32>>, Rc<List>),
    Nil
}

pub fn refcell_demo() {

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    println!("a {:?}", &a);
    println!("b {:?}", &b);
    println!("c {:?}", &c);

    *value.borrow_mut() += 10;

    println!("a {:?}", &a);
    println!("b {:?}", &b);
    println!("c {:?}", &c);

}