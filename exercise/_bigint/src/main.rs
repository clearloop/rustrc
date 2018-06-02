//extern crate hex_slice;
extern crate num_bigint;
extern crate sha2;
extern crate num;

//use hex_slice::AsHex;
use num_bigint::BigInt;
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
    println!("{:?}", String::from_utf8(b.unwrap().to_bytes_be().1).unwrap());// 转换为字符串会爆内存
    //println!("{:x}", s.result().as_hex());
    //println!("{:?}",s.result().to_vec());
    //println!("{:?}",s.result().to_vec());
}
