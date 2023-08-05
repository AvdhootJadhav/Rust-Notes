pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }
    else{
        y
    }
}

#[derive(Debug)]
pub struct ImportantExcerpt<'a>{
    pub part: &'a str
}