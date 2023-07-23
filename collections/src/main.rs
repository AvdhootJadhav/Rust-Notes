use std::collections::HashMap;

#[derive(Debug)]
enum User{
    Name(String),
    Age(i32),
    Email(String)
}

#[derive(Debug)]
struct StructUser{
    name: String,
    age: i32
}

fn main() {
    
    let str = "Hello";

    let mut map = HashMap::new();

    for s in str.chars(){
        println!("{}", s);
        let count = map.entry(s).or_insert(0);
        *count+=1;
    }

    println!("{:?}", map);
}

fn hashmap_demo(){
    let mut map = HashMap::new();
    map.insert(String::from("Avdhoot"), 21);
    map.insert(String::from("Amey"), 22);
    map.insert(String::from("Mohit"), 22);

    // entry checks if a key is present or not
    map.entry(String::from("Roshan")).or_insert(22);

    println!("{}", map.get("Roshan").unwrap());
}

fn concat_demo(){
    let mut str = String::from("Hello");
    let mut str2 = String::from("World");
    let mut str3 = str + &str2;
    println!("{}", str3);
    println!("{}", str2);

    let a = "Здравствуйте";

    println!("{}", &a[0..2]);
}

fn vectors_demo(){
    let arr = vec![1, 20, 98, 45, 67, 34];

    for i in &arr {
        print!("{} ", i);
    }
    println!();

    println!("{:?}", arr);

    let brr = vec![
        User::Name(String::from("Avdhoot")),
        User::Age(21),
        User::Email(String::from("avdhootj002@gmail.com"))
    ];

    println!("{:?}", brr);

    let crr = vec![
        StructUser{
            name: String::from("Avdhoot"),
            age: 21
        }
    ];

    println!("{:?}", crr);
}

fn array_access(){
    let arr = vec![1, 2, 3, 5, 10];
    println!("{:?}", arr);

    let mut brr:Vec<i8> = Vec::new();
    brr.push(10);
    brr.push(20);
    brr.push(1);
    brr.push(29);
    println!("{:?}", brr);

    let val = brr[4];

    println!("{}", val);

    // [] throws panic error if index out of bound
    // get uses Option so only two possible values Some or None. No errors are thrown and easy to handle

    println!("{:?}", brr);

    if brr.get(4) == None {
        println!("Index out of bounds");
    }
    else {
        println!("{:?}", brr.get(4));
    }
}
