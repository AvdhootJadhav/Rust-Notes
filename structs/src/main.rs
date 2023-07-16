struct User{
    name: String,
    email: String,
    sign_in_count: u32,
    active: bool
}

fn build_user(name: String, email: String) -> User {

    User { name, email, sign_in_count: 1, active: true }
}

fn structs_basic(){
    let user = build_user("Avdhoot".to_owned(), "avdhoot@gmail.com".to_owned());

    let user2 = User{
        sign_in_count: 2,
        ..user
    };

    // data moved from user to user2 -> user is invalid

    println!("User 1 details :");
    println!("{}",user2.name);
    println!("{}",user2.email);
    println!("{}",user2.sign_in_count);
    println!("{}",user2.active);
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.length
    }

    fn square(size: u32) -> Self{
        Self { length: size, width: size }
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.length < rect.length
    }
}

fn main() {
    
    let rectangle = Rectangle{
        length: 10,
        width: 20
    };

    let rectangle2 = Rectangle{
        length: 10,
        width: 20
    };

    println!("Rectangle: {:#?}",rectangle);

    println!("The area of rectangle is {}", rectangle.calculate_area());

    println!("Do rectangle hold another rectangle? -> {}", rectangle.can_hold(&rectangle2));

    let square = Rectangle::square(4);

    println!("Square : {:#?}", square);
}

