use std::{fs::File, io::{ErrorKind, Read, Error}};

fn main() {
    
    let result = error_propagation_shortcut();

    print!("Result {:?}", result.unwrap())
}

fn error_handling_with_files(){
    let file_result = File::open("/home/avdhoot/hello.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("Error occurred when creating file {:?}", err),
            },
            
            _ => {
                panic!("Error occurred while opening the file");
            }
        },
    };
}

fn read_username_from_file() -> Result<String, std::io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err)
    }
}

fn error_propagation_shortcut() -> Result<String, Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
