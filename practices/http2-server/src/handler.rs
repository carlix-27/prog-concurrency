use std::{
    io::{prelude::*},
    net::TcpStream,
};
use std::io::BufReader;
use crate::leibniz::calculate_pi_parallel;

pub(crate) fn read_request_line(stream: &TcpStream) -> String {
    let buf_reader = BufReader::new(stream);
    buf_reader.lines().next().unwrap().unwrap()
}

pub(crate) fn parse_request_line(request: &String) -> (String, String) {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("").to_string();
    (method, path)
}

pub(crate) fn route_request(method: &String, path: &String) -> String {
    if method == "GET" && path.starts_with("/pi/") {
        if let Some(number) = path.strip_prefix("/pi/") {
            if let Ok(i) = number.parse::<u64>() {
                let (pi, duration) = calculate_pi_parallel(i, 4);
                let body = format!(
                    "Valor de Pi para el termino {}: {}\nTiempo: {:.6} segundos",
                    i, pi, duration
                );
                return build_response("200 OK", body);
            }
        }
    }
    build_response("404 NOT FOUND", "Endpoint no encontrado".to_string())
}

fn build_response(status: &str, body: String) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    )
}