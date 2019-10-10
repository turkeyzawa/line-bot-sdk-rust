mod helpers;

#[cfg(test)]
mod tests {
    extern crate futures;
    extern crate line_bot_sdk_rust;
    extern crate tokio;

    use super::helpers::TestServer;
    use line_bot_sdk_rust::{client::Client, client::ClientConfig, message::TextMessage};

    #[test]
    fn it_works() {
        std::env::set_var("API_BASE_URL", "http://0.0.0.0:8888");

        println!("Test server is starting...");
        let server = TestServer::new();
        println!("Test server has been started.");

        let mut core = tokio::runtime::Runtime::new().unwrap();

        let client = Client::new(ClientConfig::new("hoge"));
        let future = client.push_message("hogehoge", vec![TextMessage::new("hogehoge", false)]);

        match core.block_on(future) {
            Ok(r) => println!("done! {:?}", r),
            Err(e) => println!("{:?}", e),
        };

        println!("Test server is stopping...");
        server.stop();
        println!("Test server has been stopped.");
    }
}
