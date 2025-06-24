use std::net::{TcpStream};
use std::io::Write;
use std::process;

mod poker;
use poker::cards::{Card, Suit};

fn main() {
    println!("Trying to connect to server");
    let mut socket = match TcpStream::connect("127.0.0.1:4444") {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("Failed to connect to server: {e:?}");
            process::exit(1);
        },
    };

    println!("Connected!");

    let card: Card = Card::new(14, Suit::Spades);
    println!("{card:?}");

    let message = "Hello World!";
    match socket.write_all(message.as_bytes()) {
        Err(e) => {
            eprintln!("Failed to send data: {e:?}");
        },
        _ => (),
    };
}