use std::ops::Deref;

pub fn box_demo(){
    let y = MyBox::new(10);

    assert_eq!(*y, 10);

    let msg = MyBox::new(String::from("Rust"));
    hello(&msg);

    let _ptr = CustomDropPointer{
        data: String::from("Hello")
    };

    let _ptr2 = CustomDropPointer{
        data: String::from("Bye")
    };

    println!("CustomPointer are created");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str){
    println!("Hello {name}!!")
}

struct CustomDropPointer{
    data: String
}

impl Drop for CustomDropPointer {
    fn drop(&mut self) {
        println!("Dropping the value {}", &self.data);
    }
}