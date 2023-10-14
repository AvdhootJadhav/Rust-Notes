use std::vec;

fn main() {

    let favorite_color: Option<&str> = Some("Blue");
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color {} as background", color);
    } else if is_tuesday {
        println!("Tuesday is green day")
    } else if let Ok(age) = age {
        if age > 30 {
            println!("You are old guy with age more than 30")
        }
        else {
            println!("You seem oretty young")
        }
    } else {
        println!("Using blue as background")
    }


    let mut t = Vec::new();

    t.push(10);
    t.push(20);
    t.push(100);

    while let Some(top) = t.pop() {
        println!("{}", top);
    }

    let a = vec!['a','b','c'];

    for (index, value) in a.iter().enumerate() {
        println!("{} is present at index {}", value, index)
    }

    let x = 10;

    match x {
        1..=5 => println!("x lies in range 1 to 5"),
        _ => println!("x lies outside the range")
    }

}
