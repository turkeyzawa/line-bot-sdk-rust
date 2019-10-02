extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{dev, web, App, HttpRequest, HttpResponse, HttpServer};
use futures::prelude::*;

pub struct TestServer {
    addr: dev::Server,
}

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

impl TestServer {
    pub fn new() -> Self {
        let sys = actix::System::new("testserver");
        let addr = HttpServer::new(|| App::new().route("/", web::get().to(index)))
            .bind("0.0.0.0:10010")
            .unwrap()
            .shutdown_timeout(1)
            .start();
        let _ = sys.run();
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
