mod calculate;
mod process_http_call;

use crate::process_http_call::{read_request_line, parse_request_line, route_request};

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

// GET /pi/100000 HTTP/1.1

fn handle_connection(mut stream: TcpStream) {
    let request_line = read_request_line(&stream);
    let (method, path) = parse_request_line(&request_line);
    let response = route_request(&method, &path);

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

