use std::{net::TcpStream, io::Write};
mod unixtime;

fn send(mut stream: &TcpStream, buf: &str) {
    stream.write(buf
        .as_bytes())
        .expect_err("Failed to send message to server.");
}

fn main() {
    let con = TcpStream::connect(("localhost", 8080))
                                .expect("Could not connect to server!");
    println!(">>> Connected to server {:?}", con.peer_addr().unwrap());
    
    loop {
        let client_ts = unixtime::unix_timestamp_micros()
                                        .to_string();
        send(&con, &client_ts);
        println!(">>> Client time: {}", client_ts);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}