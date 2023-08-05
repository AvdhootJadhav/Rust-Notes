use generics::tuts::lifetimes::ImportantExcerpt;

fn main() {
    let string = "Hello World. I am kaju";
    let first_sentence = string.split('.').next().expect("Could not find the sentence");
    
    let i = ImportantExcerpt{
        part: first_sentence
    };

    println!("{:#?}", i);
}
