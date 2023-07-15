fn main() {

    println!("Inside main for slice");
    let str = "Hello World";

    let substring = &str[0..5];
    println!("{}", substring);

    println!("{}",modified_first_word(str));
    let res = get_slice(str);
    assert_eq!(substring, res);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{}",item);
        // converting byte -> char
        let character: char = char::from_u32(item.into()).unwrap();
        println!("{}", character);
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn get_slice(s: &str) -> &str{

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}

fn modified_first_word(s: &str) -> usize{
    let mut i = 0;
    for ch in s.chars() {
        if ch == ' ' {
            return i
        }
        i = i+1;
    }

    s.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String){
    s.push_str(" World")
}

fn ownership(){
    let mut str = String::from("Hello");
    let len = calculate_length(&str);

    println!("{}",len);
    println!("{}",str);

    change(&mut str);

    println!("{}",str);

    let ref1 = &mut str;
    println!("{}",ref1);

    let mut sample = "sample";

    let sampleref = &sample;
    let sampleref2 = &sample;
    println!("{}, {}", sampleref, sampleref2);
    let ref3 = &mut sample;

    println!("{}", ref3)
}
