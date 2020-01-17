#![feature(type_ascription)]
#![allow(dead_code)]
use std::fs::File;
use std::io::{Read, Error};
use std::borrow::BorrowMut;
use std::result::Result;
mod parser;
use crate::parser::{parse_json_entry};
use common::USIZEWrapper;



fn main() -> Result<(), Error>{
    let mut file = File::open("./data/test1.json")?;
    let mut contents = String::new();
    file.read_to_string(contents.borrow_mut())?;
    let bytes_ref = contents.as_bytes();
    let idx = USIZEWrapper::new(0).trim_whitespace(bytes_ref);

    parse_json_entry( bytes_ref, idx)?;


    println!("=================================\n{}", contents);
    Ok(())

}
