use std::{fs::read_to_string, num::ParseIntError};

#[derive(Debug)]
struct X;

fn _errors_with_x() -> Result<(), X> {
    Err(X {})
}

#[derive(Debug)]
pub enum ReadNumberFromFileError {
    Io(std::io::Error),
    ParseInt(ParseIntError),
}

impl std::fmt::Display for ReadNumberFromFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadNumberFromFileError::Io(e) => write!(f, "IO Error: {}", e),
            ReadNumberFromFileError::ParseInt(e) => write!(f, "ParseInt Error: {}", e),
        }
    }
}

impl From<std::io::Error> for ReadNumberFromFileError {
    fn from(value: std::io::Error) -> Self {
        ReadNumberFromFileError::Io(value)
    }
}

impl From<ParseIntError> for ReadNumberFromFileError {
    fn from(value: ParseIntError) -> Self {
        ReadNumberFromFileError::ParseInt(value)
    }
}

impl std::error::Error for ReadNumberFromFileError {}

pub fn read_number_from_file(path: &str) -> Result<i32, ReadNumberFromFileError> {
    let num_str = read_to_string(path)?;
    let num: i32 = num_str.trim().parse()?;
    Ok(num)
}

/*
Downcasting! Yuck! Don't do this!

match e.downcast_ref::<std::io::Error>() {
    None => println!("No IO Error"),
    Some(_) => println!("IO Error"),
}

match e.downcast_ref::<ParseFloatError>() {
    None => println!("No ParseInt Error"),
    Some(_) => println!("ParseInt Error"),
    }
*/

pub fn call_error_handling() {
    let file_name = String::from("answer.txt");
    match read_number_from_file(&file_name) {
        Ok(num) => println!("The number is: {}", num),
        Err(ReadNumberFromFileError::Io(e)) => eprintln!("IO Error: {}", e),
        Err(ReadNumberFromFileError::ParseInt(e)) => eprintln!("ParseInt Error: {}", e),
    }
}
