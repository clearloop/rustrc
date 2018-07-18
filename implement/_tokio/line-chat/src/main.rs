extern crate line_chat;
use line_chat::tokio;
use line_chat::tokio::net::{TcpListener, TcpStream};
use line_chat::tokio::prelude::*;
use line_chat::futures::future::{self, Either};
use line_chat::shared::Shared;
use line_chat::lines::Lines;
use line_chat::peer::Peer;
use std::sync::{Arc, Mutex};


fn process(socket: TcpStream, state: Arc<Mutex<Shared>>) {
    // Wrap the socket with the `Lines` codec that we wrote above.
    let lines = Lines::new(socket);

    let connection = lines.into_future()
        .map_err(|(e, _)| e)
        .and_then(|(name, lines)| {
            let name = match name {
                Some(name) => name,
                None => {
                    return Either::A(future::ok(()));
                }
            };

            println!("`{:?}` is joining the chat", name);

            // Create the peer.
            //
            // This is also a future that processes the connection, only
            // completing when the soket closes.
            let peer = Peer::new(
                name,
                state,
                lines
            );

            // Wrap `peer` with `Either::B` to make the return type fit.
            Either::B(peer)
        }).map_err(|e| {
            println!("connection error = {:?}", e);
        });
}

fn main() {
    let state = Arc::new(Mutex::new(Shared::new()));
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    
    let server = listener.incoming().for_each(move |socket| {
        // TODO: Process socket
        
        process(socket, state.clone());
        Ok(())
    }).map_err(|_err| {
        // Handle error by printing to STDOUT.
        println!("accept error = {:?}", _err);
    });

    println!("server running on localhost:6142");
    // Start the server
    //
    // This does a few things:
    // 
    // * Start the Tokio runtime (reactor, threadpool, etc...)
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime.
    //   spawned tasks have completed.
    tokio::run(server);
}

