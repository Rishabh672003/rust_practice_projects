use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];
    while match stream.read(&mut buffer) {
        Ok(n) => match n {
            0 => {
                let _ = stream.shutdown(Shutdown::Both);
                false
            }
            _ => {
                let msg = from_utf8(&buffer[..n]).unwrap();
                println!("{}", msg);
                stream.write_all(msg.as_bytes()).unwrap();
                let _ = stream.flush();
                true
            }
        },
        Err(_) => {
            let _ = stream.shutdown(Shutdown::Both);
            false
        }
    } {}
}

fn main() {
    let address = "127.0.0.1:6969";
    println!("Server running at {address}");
    let listener = TcpListener::bind(address).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_stream(stream);
                });
            }
            Err(_) => {
                panic!("error occured")
            }
        }
    }
}
