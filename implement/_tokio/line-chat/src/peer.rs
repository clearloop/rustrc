// peers.rs
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use super::futures::{Future, Stream, Poll, Async};
use super::shared::Shared;
use super::lines::Lines;
use super::tokio::io::Error;
use super::bytes::{BytesMut, Bytes, BufMut};
use super::futures::sync::mpsc;

// Shorthand for the receive half of the message channel.
type Rx = super::futures::sync::mpsc::UnboundedReceiver<Bytes>;

struct Peer {
    /// Name of the peer. This is the first line received from the client.
    name: BytesMut,

    /// The TCP socket wrapped with the `Lines` codec.
    lines: Lines,

    /// Handle to the shared chat state.
    state: Arc<Mutex<Shared>>,

    /// Receive half of the message channel.
    ///
    /// This is used to receive messages from peers. when a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: Rx,

    /// Client socket address.
    ///
    /// The socket address is used as the key in the `peers` HashMap. The
    /// address is saved so that the `Peer` drop implementation can clean up its
    /// entry.
    addr: SocketAddr,
}

impl Peer {
    fn new(
        name: BytesMut,
        state: Arc<Mutex<Shared>>,
        lines: Lines
    ) -> Peer {
        // Get the client socket address
        let addr = lines.socket.peer_addr().unwrap();

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().unwrap()
            .peers.insert(addr, tx);

        Peer {
            name, lines, state, rx, addr,
        }
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        self.state.lock().unwrap().peers
            .remove(&self.addr);
    }
}

impl Future for Peer {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        // Receive all message from peers.
        loop {
            // Pollinng an `UnnboundedReveiver` cannot fail, so `unwrap`
            // here is safe.
            match self.rx.poll().unwrap() {
                Async::Ready(Some(v)) => {
                    // Buffer the line. Once all lines are buffered,
                    // they will be flushed to the socket (right
                    // below).
                    self.lines.buffer(&v);
                }
                _ => break,
            }
        }

        // Flush the write buffer to the socket
        let _ = self.lines.poll_flush()?;

        // Read new lines from the socket
        while let Async::Ready(line) = self.lines.poll()? {
            println!("Reveived line ({:?}) : {:?}", self.name, line);

            if let Some(message) = line {
                // Append the peer's name to the front of the line:
                let mut line = self.name.clone();
                line.put(": ");
                line.put(&message);
                line.put("\r\n");;

                // We're using `Bytes`, which allows zero-copy clones
                // (by storing the data in an Arc internally).
                //
                // However, before cloning, we must freeze the data.
                // This converts it from mutable -> immutable.
                // allowing zero copy cloning.
                let line = line.freeze();

                // Now, send the line to all other peers
                for (addr, tx) in &self.state.lock().unwrap().peers {
                    // Dont't send the message to ourselves
                    if *addr != self.addr {
                        // The send only fails if the rx half has been
                        // dropped, however this is impossible as the
                        // `tx` half will be removed from the map
                        // before the `rx` is dropped.
                        tx.unbounded_send(line.clone()).unwrap();
                    }
                }
            } else {
                // EOF was reached. The remove client hash disconnected.
                // There is nnothing more to do.
               return Ok(Async::Ready(()));
            }
        }

        // As always, it is importantn to note just return `NotReady`
        // without ensuring an liner future also returned `NotReady`.
        //
        // We know we got a `NotReady` from either `self.rx` or `self.lines`, so the contract is respected.
        Ok(Async::NotReady)
    }
}

