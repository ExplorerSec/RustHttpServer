use rust_http_server::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    let addr = "127.0.0.1:17788";
    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind(addr).unwrap();
    println!("Running on {addr}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_stream(stream));
    }

    println!("[-] MainFunc Shutting down.");
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("\n{}\n", String::from_utf8_lossy(&buffer));

    if let Ok(str) = std::str::from_utf8(&buffer) {
        if let Some((start_line, content)) = str.split_once('\r') {
            let (status_line, filename) = match start_line {
                // normal get
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK \r\n\r\n", "./html/index.html"),
                // slow get
                "GET /slow HTTP/1.1" => {
                    thread::sleep(Duration::from_secs(5));
                    ("HTTP/1.1 200 OK \r\n\r\n", "./html/index.html")
                }
                // cookie get
                "GET /cookie HTTP/1.1" => (
                    "HTTP/1.1 200 OK\r\nContent-type: text/html\r\nSet-Cookie: Expzero=Exp0\r\n\r\n",
                    "./html/index.html",
                ),
                "GET /login HTTP/1.1" => ("HTTP/1.1 200 OK \r\n\r\n", "./html/login.html"),
                "POST /login HTTP/1.1" => ("HTTP/1.1 200 OK \r\nSet-Cookie: Login=OK\r\n\r\n", ""),
                "GET /user HTTP/1.1" => {
                    if content.find("Login=OK").is_some() {
                        ("HTTP/1.1 200 OK \r\n\r\n", "./html/user.html")
                    } else {
                        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "./html/404.html")
                    }
                }
                _ => ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "./html/404.html"),
            };

            let content = if !filename.is_empty() {
                fs::read_to_string(filename).unwrap()
            } else {
                "".to_string()
            };
            let response = format!("{}{}", status_line, content);
            stream.write(&response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
