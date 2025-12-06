use std::fmt::Write as FmtWrite;
use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:0")?;
    println!("listening on {}", listener.local_addr()?);

    for conn in listener.incoming() {
        handle(conn?)?;
    }
    Ok(())
}

fn handle(mut conn: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Need to read HTTP request from the stream
    let r = BufReader::new(&mut conn);

    for line_res in r.lines() {
        let line = line_res?;
        if line.is_empty() {
            // This is the blank line between headers and body
            break;
        }
        println!("{}", line)
    }

    // TODO: read body

    // Write a response back to the stream
    let response_params = construct_ok_response("Hello world!\n".to_string());
    let resp = marshal_response(response_params);
    conn.write_all(resp?.as_bytes())?;

    Ok(())
}

struct ResponseParams {
    http_version: String,
    status_code: i32,
    reason_phrase: String,
    headers: Vec<(String, String)>,
    body: String,
}

fn construct_ok_response(body: String) -> ResponseParams {
    ResponseParams {
        http_version: "1.1".to_string(),
        status_code: 200,
        reason_phrase: "OK".to_string(),
        headers: vec![
            ("Content-Type".to_string(), "text/plain".to_string()),
            (
                "Content-Length".to_string(),
                body.as_bytes().len().to_string(),
            ),
        ],
        body: body,
    }
}

fn marshal_response(params: ResponseParams) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!(
        "HTTP/{http_version} {status_code} {reason_phrase}\r\n\
{headers}\
\r\n\
{body}",
        http_version = params.http_version,
        status_code = params.status_code,
        reason_phrase = params.reason_phrase,
        headers = marshal_headers(params.headers)?,
        body = params.body,
    ))
}

fn marshal_headers(headers: Vec<(String, String)>) -> Result<String, Box<dyn std::error::Error>> {
    let mut s = String::new();
    for kv in headers {
        let (key, val) = kv;
        write!(s, "{key}: {val}\r\n")?;
    }
    Ok(s)
}
