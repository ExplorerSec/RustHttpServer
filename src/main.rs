use std::{fs, io::{Read, Write}, net::{TcpListener, TcpStream}, thread, time::Duration};
fn main() {
    let addr ="127.0.0.1:17788";
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        
        thread::spawn(||{
            handle_stream(stream)
        });
    }
}

fn handle_stream(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("{}",String::from_utf8_lossy(&buffer));

    // normal get
    let get =b"GET / HTTP/1.1";
    // slow get
    let get_slow =b"GET /slow HTTP/1.1";

    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK \r\n\r\n","./html/index.html")
    }else if buffer.starts_with(get_slow){
        // slow get (use sleep)
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK \r\n\r\n","./html/index.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n","./html/404.html")
    };

    let content =fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line,content);
    stream.write(&response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
