use std::{net::{TcpListener, TcpStream}, io::Read};
mod unixtime;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let server_time = unixtime::unix_timestamp_micros();
    let client_time = String::from_utf8_lossy(&buffer[..])
                                .into_owned()
                                .parse::<u128>()
                                .unwrap();
                                
    println!(">>> Client time: {}", client_time);
}

fn main() {
    println!(">>> Server running on localhost:8080");
    let listener = TcpListener::bind("localhost:8080").unwrap();
    println!(">>> Listening ...");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}