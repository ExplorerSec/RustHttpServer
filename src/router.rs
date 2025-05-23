use crate::easy_auth::Auth;

use std::{fs, io::prelude::*, net::TcpStream, thread, time::Duration};

enum HttpStatus {
    Status200,
    Status404,
    Status403,
}
enum HttpBody {
    Text(&'static str),
    File(&'static str),
    //FILE(String),
    None,
}

fn pack_http_response(status: HttpStatus, str: &str, body: HttpBody) -> String {
    let str_status = match status {
        HttpStatus::Status200 => "HTTP/1.1 200 OK",
        HttpStatus::Status403 => "HTTP/1.1 403 FORBIDDEN",
        HttpStatus::Status404 => "HTTP/1.1 404 NOT FOUND",
    };
    let str_body = match body {
        HttpBody::None => String::from(""),
        HttpBody::Text(str) => str.to_string(),
        HttpBody::File(filename) => fs::read_to_string(filename).unwrap(),
    };
    format!("{} \r\n{}\r\n\r\n{}", str_status, str, str_body)
}

fn login(str: &str, auth: Auth) -> bool {
    if let Some((_, usr_pwd)) = str.rsplit_once('\n') {
        if let Some((usr, pwd)) = usr_pwd.rsplit_once('~') {
            let auth = auth.lock().unwrap();
            return auth.auth_accounts(usr.to_string(), pwd.trim_matches('\0').to_string()).is_ok();
        }
    }

    false
}

pub fn handle_stream(mut stream: TcpStream, auth: Auth) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("\n{}\n", String::from_utf8_lossy(&buffer));

    if let Ok(str) = std::str::from_utf8(&buffer) {
        if let Some((start_line, content)) = str.split_once('\r') {
            if let Some((method_path, _)) = start_line.rsplit_once(' ') {
                let (status, head, body) = match method_path {
                    "GET /" => (
                        HttpStatus::Status200,
                        "",
                        HttpBody::File("./html/index.html"),
                    ),
                    "GET /login" => (
                        HttpStatus::Status200,
                        "",
                        HttpBody::File("./html/login.html"),
                    ),
                    "POST /login" => {
                        if login(content, auth) {
                            (
                                HttpStatus::Status200,
                                "Set-Cookie: Login=OK",
                                HttpBody::None,
                            )
                        } else {
                            (HttpStatus::Status403, "", HttpBody::Text("Failed"))
                        }
                    }
                    "GET /user" => {
                        if content.contains("Login=OK") {
                            (
                                HttpStatus::Status200,
                                "",
                                HttpBody::File("./html/user.html"),
                            )
                        } else {
                            (HttpStatus::Status403, "", HttpBody::Text("Please log in!"))
                        }
                    }
                    "GET /slow" => {
                        thread::sleep(Duration::from_secs(5));
                        (
                            HttpStatus::Status200,
                            "",
                            HttpBody::File("./html/index.html"),
                        )
                    }
                    "CONNECT cn.bing.com:443" => {
                        (HttpStatus::Status200, "", HttpBody::Text("Hello"))
                    }
                    _ => (HttpStatus::Status404, "", HttpBody::File("./html/404.html")),
                };
                let response = pack_http_response(status, head, body);
                stream.write(&response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }
}
