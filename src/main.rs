#![feature(type_ascription)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use std::fs::File;
use std::io::{Read, Error};
use std::borrow::BorrowMut;
use std::result::Result;
mod parser;
use crate::parser::{parse_json_entry};
use common::USIZEWrapper;
use time::{Instant};
#[cfg(feature = "mul-dbg")]
use common::{mul_dbg};



fn main() -> Result<(), Error>{
    let now = Instant::now();

    let mut file = File::open("./data/test1.json")?;
    let mut contents = String::new();
    let _ = file.read_to_string(contents.borrow_mut())?;
    let bytes_ref = contents.as_bytes();
    let mut idx = USIZEWrapper::new(0);
    let idx_ref = idx.borrow_mut().trim_whitespace(bytes_ref);

    let obj = parse_json_entry(bytes_ref, idx_ref)?;


    dbg!(now.elapsed().as_seconds_f64());

    #[cfg(feature = "mul-dbg")]
    mul_dbg!("=====================================\nParsing res: {}", obj);
    Ok(())
}
