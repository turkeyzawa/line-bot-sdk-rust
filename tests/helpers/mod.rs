extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{dev, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::prelude::*;
use std::sync::mpsc;

pub struct TestServer {
    addr: dev::Server,
}

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

impl TestServer {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let sys = actix::System::new("http-server");

            let host = std::env::var("API_BASE_URL").unwrap();
            let addr = HttpServer::new(|| App::new().route("/", web::get().to(index)))
                .bind(host.replace("https://", "").replace("http://", ""))
                .unwrap()
                .shutdown_timeout(5)
                .start();

            let _ = tx.send(addr);
            let _ = sys.run();
        });
        let addr = rx.recv().unwrap();
        Self { addr }
    }
    pub fn stop(&self) -> () {
        let _ = self
            .addr
            .stop(false)
            .wait()
            .map(|_| println!("Test server was stopped."));
    }
}
