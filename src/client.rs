extern crate futures;
extern crate reqwest;
use super::constants::LINE_REQUEST_ID_HTTP_HEADER_NAME;
use super::message::{Message, SendMessage};
use super::response::MessageAPIResponse;
use futures::prelude::*;
use reqwest::r#async::{Client as ReqwestClient, Response};
pub use reqwest::Error;

#[derive(Clone)]
pub struct ClientConfig {
    channel_access_token: &'static str,
}

#[derive(Clone)]
pub struct Client {
    config: ClientConfig,
    http_client: ReqwestClient,
    host: &'static str,
}

impl Client {
    fn parse_message_response(res: Response) -> Result<MessageAPIResponse, Error> {
        let request_id = match res.headers().get(LINE_REQUEST_ID_HTTP_HEADER_NAME) {
            Some(header_val) => match header_val.to_str() {
                Ok(val) => Some(val.to_string()),
                _ => None,
            },
            None => None,
        };
        Ok(MessageAPIResponse::new(request_id))
    }

    pub fn new(config: ClientConfig) -> Self {
        // FIXME: Dummy URL
        let host = "https://ptsv2.com/t/wq1jm-1569689108";
        let http_client = ReqwestClient::builder()
            .build()
            .expect("failed to create http client.");

        Self {
            config,
            http_client,
            host,
        }
    }

    pub fn push_message<T: Message>(
        &self,
        to: &'static str,
        messages: Vec<T>,
    ) -> impl Future<Item = MessageAPIResponse, Error = Error> {
        let body = SendMessage::new(to, messages);
        self.http_client
            .post(&format!("{}/post", self.host))
            .json(&body)
            .send()
            .and_then(Client::parse_message_response)
    }
}

#[cfg(test)]
mod tests {
    extern crate futures;
    extern crate tokio;
    use super::super::message::TextMessage;
    use super::{Client, ClientConfig};

    #[test]
    fn it_works() {
        let mut core = tokio::runtime::Runtime::new().unwrap();

        let client = Client::new(ClientConfig {
            channel_access_token: "test",
        });
        let future = client.push_message("hogehoge", vec![TextMessage::new("hogehoge")]);

        match core.block_on(future) {
            Ok(r) => println!("done! {:?}", r),
            Err(e) => println!("{:?}", e),
        };
    }
}
