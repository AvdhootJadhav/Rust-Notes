#[derive(Debug)]
enum IpAddress{
    V4(String),
    V6(String)
}

impl IpAddress {
    fn greet(&self){
        println!("Hello enum user")
    }
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}


fn main() {

    let coin = Coin::Nickel;

    match coin {
        Coin::Penny => println!("{}",1),
        Coin::Nickel => println!("{}",5),
        Coin::Dime => println!("{}",10),
        Coin::Quarter => println!("{}",25)
    };

    let value = Some(10);
    let new_value = increment(value);

    println!("{:?}", new_value);

    game_logic(3);
}

fn game_logic(roll:i32){
    match roll {
        3 => println!("You got a new hat"),
        7 => println!("Oops! you lost your new hat"),
        _ => println!("You got a reroll")
    }
}

fn increment(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(i+1) 
    }
}

fn enum_demo(){
    let four = IpAddress::V4(String::from("127.0.0.1"));
    let six = IpAddress::V6(String::from("::1"));

    println!("{:#?}", four.greet());
    println!("{:#?}", six);
}
