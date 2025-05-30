use std::{
    net::TcpListener,
    sync::{Arc, Mutex},
};

mod utils;
use utils::thread_pool::ThreadPool;

mod web;
use web::auth::Auth ;
use web::router::handle_stream;

fn main() {
    // let auth = new_auth("data/account.ini");
    let auth = Arc::new(Mutex::new(Auth::init()));

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
