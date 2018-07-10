extern crate secp256k1;
extern crate rand;
    
use secp256k1::{PublicKey, Secp256k1, SecretKey, Message};

fn main() {

    let curve = Secp256k1::new();    
    let _rand = rand::random::<[u8;32]>();
    let _rand2 = rand::random::<[u8;32]>();    
    let priv_key = SecretKey::from_slice(&curve, &_rand).unwrap();
    let _pub_key = PublicKey::from_secret_key(&curve, &priv_key).unwrap();
        
    let secp = Secp256k1::new();
    let msg = Message::from_slice(&_rand).unwrap();
    let _msg = Message::from_slice(&_rand2).unwrap();
    
    let _sig = secp.sign_schnorr(&msg, &priv_key).unwrap();
    //let _verify = secp.verify_schnorr(&msg, &_sig, &_pub_key);
    let _pub_key2 = secp.recover_schnorr(&_msg, &_sig).unwrap();

    let _verify = secp.verify_schnorr(&msg, &_sig, &_pub_key);
    //assert_eq!(_pub_key, _pub_key2);
    let _v = _pub_key.eq(&_pub_key2);
    println!("{:?}", _v);
        
    println!("{:?}", _verify.is_ok());
}
