use std::{net::TcpStream, io::Write};
use clap;

mod unixtime;

fn send(mut stream: &TcpStream, buf: &str) -> std::io::Result<()> {
    println!("Sending value: {}", &buf);
    stream.write(buf
        .as_bytes())
        .expect("ERROR: cannot send data to server");
    Ok(())
}

fn main() {
    let matches = clap::App::new("Systime-Server")
        .arg(clap::Arg::with_name("ip")
            .short('i')
            .long("ip")
            .value_name("IP")
            .help("IP address to listen on")
            .takes_value(true))
        .arg(clap::Arg::with_name("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Port to listen on")
            .takes_value(true))
        .get_matches(); 
    
    let ip = matches.value_of("ip").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("8080").parse::<u16>().unwrap();
    println!(">>> Connecting to server @ {ip}:{port}");
    //println!(">>> Connected to server {:?}", con.peer_addr().unwrap());
    loop {
        let con = TcpStream::connect((ip, port))
                                    .expect("Could not connect to server!");
        let client_ts = unixtime::unix_timestamp_micros()
                                        .to_string();
        send(&con, &client_ts).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}