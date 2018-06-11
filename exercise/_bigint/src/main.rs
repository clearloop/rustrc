//extern crate hex_slice;
extern crate num_bigint;
extern crate sha2;
extern crate num;
extern crate hex;

//use hex_slice::AsHex;
use num_bigint::{Sign, BigInt};
use sha2::{Sha256, Digest};
use num::traits::FromPrimitive;

fn main () {
    let i = BigInt::from(1);
    let b = BigInt::from_i64(128); //占位大于 i32 的 BigInt, 在最后一个输出出现 panic;
    let mut s = Sha256::default();
    s.input(&"hello".to_string().into_bytes());

    println!("{:?}", String::from_utf8(i.to_bytes_be().1).unwrap());
    
    println!("{:x}", s.result()); // 无法打印 Binary, Octal 只能打印 Hex; 想把 `s.result` 转为字符串;
    
    println!("{:?}", b.clone().unwrap().to_bytes_be().1);
    println!("{:?}", unsafe {String::from_utf8_unchecked(b.clone().unwrap().to_bytes_be().1)});
    //println!("{:?}", String::from_utf8(b.unwrap().to_bytes_be().1).unwrap());// 转换为字符串会爆内存
    //println!("{:x}", s.result().as_hex());
    //println!("{:?}",s.result().to_vec());
    //println!("{:?}",s.result().to_vec());


    let _b = BigInt::from(1);//.shl(256 as usize - 24 as usize );;
    let _s = BigInt::from_bytes_be(Sign::Plus, &"1".to_string().into_bytes());
    let ss = String::from_utf8(_s.to_bytes_be().1).unwrap();

    let mut h = Sha256::new();
    h.input(b"1");

    let mut h2 = Sha256::new();
    h2.input(&"1".to_string().into_bytes());
    
    println!("BigInt from_i32: ===> {:x}", _b);
    println!("BigInt from_bytes_be(string => bytes): ===> {:?}", _s.to_string());
    println!("String from(BigInt => bytes): ===> {:?}", ss);
    println!("Hash: ===> {:?}", hex::encode(h.clone().result()));
    println!("Hash2: ===> {:?}", hex::encode(h2.result())); 

    // how to convert Sha256 hex to bytes, conver to bigint and then convert back it? here is the solution.
    let bh = BigInt::from_bytes_be(Sign::Plus, &hex::encode(h.result()).into_bytes());
    println!("BigInt from hash bytes => to bytes to string(not string from just hash bytes): ===> {:?}", String::from_utf8(bh.to_bytes_be().1).unwrap());    
}
