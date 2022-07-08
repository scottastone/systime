use std::{net::TcpStream, io::Write};
mod unixtime;

fn send(mut stream: &TcpStream, buf: &str) -> std::io::Result<()> {
    println!("Sending value: {}", &buf);
    stream.write(buf
        .as_bytes())
        .expect(("ERROR: cannot send data to server"));
    Ok(())
}

fn main() {
    
    //println!(">>> Connected to server {:?}", con.peer_addr().unwrap());
    loop {
        let con = TcpStream::connect(("localhost", 8080))
                                    .expect("Could not connect to server!");
        let client_ts = unixtime::unix_timestamp_micros()
                                        .to_string();
        send(&con, &client_ts).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}