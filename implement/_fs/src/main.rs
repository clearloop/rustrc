extern crate bincode;

use std::io::prelude::*;
use std::fs::File;
use bincode::{serialize, deserialize};

#[macro_use]
pub extern crate serde_derive;
pub extern crate serde;

#[derive(Serialize, Deserialize, Debug)]
struct T {
    a: String,
    b: String
}

fn main() -> std::io::Result<()> {

    // WRITE //
    let mut buffer = File::create("foo")?;
    let f = T { a: "a".to_string(), b: "b".to_string() };
    buffer.write(&serialize(&f).unwrap())?;

    // READ //
    let mut f = File::open("foo")?;
    let mut _f_buffer = vec![];
    //let mut _f_buffer = vec![0;10];
    //let mut _f_buffer = vec![0;64];
    f.read_to_end(&mut _f_buffer)?;

    let _ret:T = deserialize(&_f_buffer[..]).unwrap();
    println!("{:?}", _f_buffer);
    println!("{:?}", _ret);
    
    Ok(())
}
