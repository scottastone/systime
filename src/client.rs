use std::{net::TcpStream, io::Write};
use clap;
mod unixtime;

fn send(mut stream: &TcpStream, buf: &str) -> std::io::Result<()> {
    let peer_ip = stream.peer_addr().unwrap().to_string();
    stream.write(buf
        .as_bytes())
        .expect("ERROR: cannot send data to server");

    println!(">>> [{peer_ip}] {buf}");
    Ok(())
}

fn main() {
    let matches = clap::App::new("Systime-Client")
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
        .arg(clap::Arg::with_name("delay")
            .short('d')
            .long("delay")
            .value_name("DELAY")
            .help("Delay between packets")
            .takes_value(true))
        .get_matches(); 
    
    let ip: &str = matches.value_of("ip").unwrap_or("localhost");
    let port: u16 = matches.value_of("port").unwrap_or("8080").parse::<u16>().unwrap();
    let delay = matches.value_of("delay").unwrap_or("100").parse::<u64>().unwrap();

    println!(">>> Connecting to server @ {ip}:{port}");
    loop {
        let con = TcpStream::connect((ip, port))
                                    .expect("Could not connect to server!");
        let client_ts = unixtime::unix_timestamp_micros()
                                        .to_string();
        send(&con, &client_ts).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }

}