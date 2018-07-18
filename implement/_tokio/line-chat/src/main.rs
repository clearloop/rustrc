extern crate line_chat;
use line_chat::tokio;
//use line_chat::tokio::io;
use line_chat::tokio::net::{TcpListener, TcpStream};
use line_chat::tokio::prelude::*;
//use line_chat::futures::future::{self, Either};
use line_chat::shared::Shared;
use line_chat::lines::Lines;
//use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn process(socket: TcpStream, state: Arc<Mutex<Shared>>) {
    // Wrap the socket with the `Lines` codec that we wrote above.
    let lines = Lines::new(socket);

    // The first line is treated as the clientn's name. The client
    // is not added to the set of connected peers until this line
    // is received.
    //
    // We use the `into_future` combinator to extract the first
    // item from the lines stream. `into_future` takes a `Stream`
    // and converts it to a future of `(first, rest)` where `rest`
    // is the original stream instannce.
    // let connection = lines.into_future()
    // // `into_future` doesn't have the right error type, so map
    // // the error to make it work.
    //     .map_err(|(e, _)| e)
    // // Process the first reveiced line as the client's name.
    //     .and_then(|(name, lines)| {
    //         let name = match name {
    //             Some(name) => name,
    //             None => {
    //                 // _TODO:_ Handle a client that disconnects
    //                 // early.
    //                 unimplemented!();
    //             }
    //         };
    //         // TODO: Rest of the process function.
    //     });
}

fn main() {
    let _state = Arc::new(Mutex::new(Shared::new()));
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    
    let server = listener.incoming().for_each(move |_socket| {
        // TODO: Process socket
        
        // process(socket, state.clone());
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

