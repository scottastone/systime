use std::{net::{TcpListener, TcpStream}, io::Read, ops::Deref, str::from_utf8};
mod unixtime;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    //let client_time_x = String::from_utf8_lossy(&buffer[..]).as_bytes().to_vec();
    let client_time = std::str::from_utf8(&buffer[..16])
                                    .unwrap()
                                    .trim()
                                    .parse::<u128>()
                                    .unwrap();
    //println!("{:?}", client_time);

    let server_time = unixtime::unix_timestamp_micros();
    let diff: f64 = (server_time - client_time) as f64 / 1000.0;
    println!(">>> Client: {client_time}µs, Server: {server_time}µs, Ping: {diff}ms");
}

fn main() {
    println!(">>> Server running on localhost:8080");
    let listener = TcpListener::bind("localhost:8080").unwrap();
    println!(">>> Listening ...");
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }

}