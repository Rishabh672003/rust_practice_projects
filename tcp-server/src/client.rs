use std::{
    io::{self, Read, Write},
    net::TcpStream,
    str::from_utf8,
};

fn main() {
    match TcpStream::connect("127.0.0.1:6969") {
        Ok(mut client) => loop {
            let mut buffer = String::new();
            let n = io::stdin().read_line(&mut buffer).unwrap();
            if n == 0 {
                break;
            }
            if client.write_all(buffer.trim().as_bytes()).is_err() {
                eprintln!("Failed to send message. Server might be down.");
                break;
            }

            if client.flush().is_err() {
                eprintln!("Failed to flush stream. Server might have disconnected.");
                break;
            }

            let mut response = [0u8; 100];
            match client.read_exact(&mut response) {
                Ok(_) => {
                    if response == buffer.as_bytes() {
                        println!("Response OK")
                    }
                }
                Err(e) => eprintln!("{e}"),
            }

            print!("Message sent: {buffer}");
            buffer.clear();
        },
        Err(e) => eprintln!("Failed to connect to server: {}", e),
    }
}
