use crate::easy_auth::Auth;

use core::str;
use std::{collections::HashMap, fs, io::prelude::*, net::TcpStream, thread, time::Duration};

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
    #[cfg(debug_assertions)]{
        println!("Login router/login");
        println!("---> text:{}",str);
    }

    if let Some((usr, pwd)) = str.rsplit_once('~') {
        let auth = auth.lock().unwrap();
        return auth.auth_accounts(usr.to_string(), pwd.trim_matches('\0').to_string()).is_ok();
    }
    false
}

struct HttpMessage{
    start_line:String,
    head:HashMap<String,String>,
    body:Vec<u8>
}

impl HttpMessage {
    pub fn _from_str(str:&str) -> Option<HttpMessage>{
        if let Some((start_line,rest_raw)) = str.split_once("\r\n"){
            if let Some((head_raw,body)) =  rest_raw.split_once("\r\n\r\n"){
                let mut map:HashMap<String,String> = HashMap::new();
                let spilt = head_raw.split("\r\n");
                for line in spilt{
                    if let Some((k,v)) =line.split_once(':'){
                        map.insert(k.to_string(), v.trim_start().to_string()); // trim_start 去除冒号后面可选的空格
                    }
                }
                return Some(HttpMessage{
                    start_line:start_line.to_string(),
                    head:map,
                    body:Vec::from(body),
                });
            }
        }
        None
    }
    pub fn from_u8(buffer:&[u8]) -> Option<HttpMessage>{
        let mut first:&[u8]=&[];
        let mut rest:&[u8]=&[];
        for i in 0..buffer.len()-3{
            if &buffer[i..i+4] == b"\r\n\r\n"{
                first = &buffer[0..i];
                rest = &buffer[i+4..buffer.len()];
                break;
            }
        }
        if let Ok(raw) = std::str::from_utf8(first){
            if let Some((start_line,head)) = raw.split_once("\r\n"){
                let mut map =HashMap::new();
                let spilt = head.split("\r\n");
                for line in spilt{
                    if let Some((k,v)) =line.split_once(':'){
                        map.insert(k.to_string(), v.trim_start().to_string()); // trim_start 去除冒号后面可选的空格
                    }
                }
                // 根据 Content-Length 调整 body 的长度
                if let Some(content_len) = map.get("Content-Length"){
                   // 使用 min 防止溢出，这里后续可以进行截断判断
                   let content_len = std::cmp::min(rest.len(),content_len.parse().unwrap_or(0));
                   rest = &rest[0..content_len];
                }
                return Some(HttpMessage{
                    start_line:start_line.to_string(),
                    head:map,
                    body:Vec::from(rest)
                });
            }
        }
        None
    }

}


pub fn handle_stream(mut stream: TcpStream, auth: Auth) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("\n{}\n", String::from_utf8_lossy(&buffer));
    
    /* if let Ok(str) = std::str::from_utf8(&buffer) {
        if let Some((start_line, content)) = str.split_once("\r\n") {
            */
    if let Some(msg) = HttpMessage::from_u8(&buffer){
        let start_line = msg.start_line;
        let content = str::from_utf8(&msg.body).unwrap_or("");

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
                        let mut resp = (HttpStatus::Status403, "", HttpBody::Text("Please log in!"));
                        if let Some(s) = msg.head.get("Cookie"){
                            if s.contains("Login=OK") {
                                resp = (
                                    HttpStatus::Status200,
                                    "",
                                    HttpBody::File("./html/user.html"),
                                );
                            } 
                        }
                        resp   
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
    //}
}
