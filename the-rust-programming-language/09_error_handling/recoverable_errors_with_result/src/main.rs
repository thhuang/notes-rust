use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "hello1.txt";
    let f = match File::open(filename) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => file,
                Err(err) => panic!("File::create failed: {:?}", err),
            },
            err => panic!("File::open failed: {:?}", err),
        },
    };

    let filename = "hello2.txt";
    let f = File::open(filename).expect("File::open failed");

    // A Shortcut for Propagating Errors: the ? Operator
    let filename = "hello3.txt";
    match read_username_from_file_version1(filename) {
        Ok(_) => {}
        Err(err) => println!("read_username_from_file failed: {:?}", err),
    };

    let filename = "hello3.txt";
    match read_username_from_file_version2(filename) {
        Ok(_) => {}
        Err(err) => println!("read_username_from_file failed: {:?}", err),
    };

    let filename = "hello3.txt";
    match read_username_from_file_version3(filename) {
        Ok(_) => {}
        Err(err) => println!("read_username_from_file failed: {:?}", err),
    };

    // We can use `?` when the return type of the main function is `Result<(), Box<dyn std::error::Error>>`
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file_version1(filename: &str) -> Result<String, Error> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_version2(filename: &str) -> Result<String, Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_version3(filename: &str) -> Result<String, Error> {
    fs::read_to_string(filename)
}
