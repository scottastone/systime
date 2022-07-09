use std::{net::{TcpListener, TcpStream}, io::Read};
use clap;
use local_ip_address::{self, local_ip};

mod unixtime;

fn calc_diff(ts1: u128, ts2: u128) -> u128 {    
    if ts1 > ts2 {
        return ts1 - ts2;
    } else {
        return ts2 - ts1;
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let client_time = std::str::from_utf8(&buffer[..16])
                                    .unwrap()
                                    .trim()
                                    .parse::<u128>()
                                    .unwrap();

    let server_time = unixtime::unix_timestamp_micros();
    let diff: f64 = calc_diff(client_time, server_time) as f64 / 1000.0;
    println!(">>> Client: {client_time}µs, Server: {server_time}µs, Ping: {diff}ms");
}

fn main() {
    // Take an argument from the command line giving the IP
    // address and port to listen on.
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
    
    let local_ip = local_ip().unwrap().to_string();
    let ip = matches.value_of("ip").unwrap_or(&local_ip);
    let port = matches.value_of("port").unwrap_or("8080");

    println!(">>> Server running on {ip}:{port}");
    let _ip_with_port = format!("{ip}:{port}");
    let listener = TcpListener::bind(_ip_with_port).unwrap();
    println!(">>> Listening ...");
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }

}