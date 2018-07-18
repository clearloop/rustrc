// peers.rs
use std::net::SocketAddr;

// Shorthand for the receive half of the message channel.
type Rx = super::futures::sync::mpsc::UnboundedReceiver<bytes::Bytes>;

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
