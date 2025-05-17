mod thread_pool;
use thread_pool::ThreadPool;

mod router;
use router::handle_stream;

use std::net::TcpListener;

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

