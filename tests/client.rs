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
        TestServer::run();

        let mut core = tokio::runtime::Runtime::new().unwrap();

        let client = Client::new(ClientConfig::new("hoge"));
        let future = client.push_message("hogehoge", vec![TextMessage::new("hogehoge")]);

        match core.block_on(future) {
            Ok(r) => println!("done! {:?}", r),
            Err(e) => println!("{:?}", e),
        };

        TestServer::stop();
    }
}
