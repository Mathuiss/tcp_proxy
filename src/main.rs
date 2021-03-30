use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const BUFSIZE: usize = 8;

fn handle_client(mut incoming: TcpStream) {
    let request = stream_read_to_end(&mut incoming);
    // save_to_file(&request);

    let mut upstream =
        TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to upstream host");

    upstream
        .write_all(&request.as_bytes())
        .expect("Failed to write request to stream");

    upstream.flush().unwrap();
    upstream
        .set_read_timeout(Some(Duration::from_millis(25)))
        .expect("Failed to set read timeout");

    let response = stream_read_to_end(&mut upstream);

    match incoming.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn stream_read_to_end(stream: &mut TcpStream) -> String {
    let mut bytes: Vec<u8> = Vec::new();

    loop {
        let mut buf = [0x0; BUFSIZE];

        // If read operation times out, EOF has been reached
        let c = match stream.read(&mut buf) {
            Ok(v) => v,
            Err(_) => break,
        };

        bytes.extend_from_slice(&buf[..c]);

        if c < BUFSIZE {
            break;
        }
    }

    return String::from(String::from_utf8_lossy(&bytes));
}

fn save_to_file(data: &String) {
    // save req to file
    let file_name = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    match File::create(format!("{}.http", file_name)) {
        Ok(mut f) => {
            match f.write(data.as_bytes()) {
                Err(e) => println!("{}", e),
                _ => {}
            };
        }
        Err(_) => panic!("Collision {}.http already exists", file_name),
    };
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Listening for connections on port 8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
