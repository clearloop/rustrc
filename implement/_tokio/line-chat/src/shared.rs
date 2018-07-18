// lib.rs

use std::net::SocketAddr;
use std::collections::HashMap;
//use std::sync::{Arc, Mutex};
use super::Tx;

#[derive(Clone)]
pub struct Shared {
    pub peers: HashMap<SocketAddr, Tx>,
}

impl Shared{
    pub fn new() -> Shared {
        return Shared{
            peers: HashMap::new()
        }
    }
}

