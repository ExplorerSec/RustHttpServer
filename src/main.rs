mod thread_pool;
use thread_pool::ThreadPool;

mod router;
use router::handle_stream;

use std::{net::TcpListener, sync::Arc};

mod easy_auth;
use easy_auth::new_auth;

fn main() {
    let auth = new_auth();

    let addr = "127.0.0.1:17788";
    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind(addr).unwrap();
    println!("Running on {addr}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let auth = Arc::clone(&auth);
        pool.execute(|| handle_stream(stream, auth));
    }

    println!("[-] MainFunc Shutting down.");
}
