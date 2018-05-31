extern crate bytes;

use bytes::Bytes;

fn main() {
    /* */
    // "a".as_bytes: [u8];
    // u8 and str?
    // get Bytes from string;
    let b1 = "str".bytes();
    let b2 = "str".as_bytes();
    println!("{:?}", b1);
    println!("{:?}", b2);
    
    /* Binary System | Octal number System | Decimal system | Hexadecimal */
    // 123: i32;
    // "123": String;
    // get Bytes from number;
    // bytes
    let b3 = Bytes::from_static("str".as_bytes());
    println!("{:?}", b3);
    
    // convert bytes to string;
    

}
