use std::fmt::Error;

pub fn error_printer(error: std::io::Error){
    eprintln!("Error : {:?}",error);
}