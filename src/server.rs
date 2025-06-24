use std::io::Read;
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::{thread, process};

fn main() {
    println!("Starting server");
    let socket = match TcpListener::bind("0.0.0.0:4444") {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("Failed to initialise socket: {e:?}");
            process::exit(1);
        }
    };

    loop {
        let incoming = socket.accept();
        if incoming.is_err() {
            let err = incoming.unwrap_err();
            eprintln!("Incoming connection failed to establish properly {err:?}");
            continue;
        }

        let (conn, addr) = incoming.unwrap();

        // New connection
        _ = thread::spawn(move || {
            // Spawn a new worker thread to look after this socket
            socket_worker(conn, addr);
        });
    }
}

fn socket_worker(mut socket: TcpStream, addr: SocketAddr) {
    println!("New connection from {addr:?}");
    loop {
        // Read from the socket
        let mut buffer = [0; 1024];
        match socket.read(&mut buffer) {
            Err(e) => {
                println!("Connection with {addr:?} has failed: {e:?}");
                return;
            },
            Ok(bytes) => {
                if bytes > 0 {
                    println!("{bytes:?} bytes from {addr:?}:");
                } else {
                    println!("Connection with {addr:?} closed");
                    return;
                }
            },
        }
        let message = String::from_utf8_lossy(&buffer);
        println!("{message}");
    }
}