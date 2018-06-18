extern crate secp256k1;
extern crate rand;
    
use secp256k1::{PublicKey, Secp256k1, SecretKey, Message};

fn main() {

    let curve = Secp256k1::new();    
    let _rand = rand::random::<[u8;32]>();
    let priv_key = SecretKey::from_slice(&curve, &_rand).unwrap();
    let _pub_key = PublicKey::from_secret_key(&curve, &priv_key).unwrap();
        
    let secp = Secp256k1::new();
    let msg = Message::from_slice(&_rand).unwrap();

    
    let _sig = secp.sign_schnorr(&msg, &priv_key).unwrap();
    let _verify = secp.verify_schnorr(&msg, &_sig, &_pub_key);
    println!("{:?}", _verify.is_ok());
}
