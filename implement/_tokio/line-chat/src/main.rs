extern crate line_chat;
use line_chat::tokio;
//use line_chat::tokio::io;
use line_chat::tokio::net::{TcpListener, TcpStream};
use line_chat::tokio::prelude::*;
//use line_chat::futures::future::{self, Either};
use line_chat::shared::Shared;

//use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//fn process(socket: TcpStream, state: Arc<Mutex<Shared>>) {
//    // Define the task that processes the connection.
//    let task = unimplemented!();
//
//    // Spawn the task
//    tokio::spawn(task);
//}        

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

