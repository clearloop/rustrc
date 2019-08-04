#![allow(dead_code)]
#![allow(unused_imports)]
mod behaviour;
mod handler;
mod error;
mod protocol;

use futures::prelude::*;
use libp2p::{
    Swarm,
    PeerId,
    identity,
    build_development_transport
};
use libp2p::kad::{ Kademlia, KademliaConfig, KademliaEvent, GetClosestPeersError };
use libp2p::kad::record::store::MemoryStore;
use std::time::Duration;

struct Node {
    keypair: identity::Keypair,
    peer_id: PeerId,
}

impl Node {
    fn new() -> Node {
        let kp = identity::Keypair::generate_ed25519();
        println!("{:?}", kp.public().into_peer_id());
        Node {
            keypair: kp.to_owned(),
            peer_id: PeerId::from(kp.public())
        }
    }
}


use behaviour::Behaviour;
pub fn main() {
    env_logger::init();
    
    let node = Node::new();

    // Manage peers and events.
    let transport = build_development_transport(node.keypair);
    let behaviour = Behaviour::new();
    let mut swarm = Swarm::new(transport, behaviour, node.peer_id);

    // Dail
    if let Some(addr) = std::env::args().nth(1) {
        let remote_addr = addr.clone();
        match addr.parse() {
            Ok(remote) => {
                match Swarm::dial_addr(&mut swarm, remote) {
                    Ok(()) => println!("Dialed {:?}", remote_addr),
                    Err(e) => println!("Dialing {:?} failed with: {:?}", remote_addr, e)
                }
            },
            Err(err) => println!("Failed to parse address to dial: {:?}", err),
        }
    }

    // listen
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();
    
    // Kick it off!
    tokio::run(futures::future::poll_fn(move || {
        loop {
            match swarm.poll().expect("Error while polling swarm") {
                Async::Ready(Some(msg)) => {
                    println!("{:?}", msg);
                },
                Async::Ready(None) | Async::NotReady  => {
                    if let Some(a) = Swarm::listeners(&swarm).next() {
                        println!("listening on {:?}", a);
                    }
                    return Ok(Async::NotReady);
                },
            }
        }
    }));
}
