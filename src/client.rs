extern crate futures;
extern crate reqwest;
use super::constants::LINE_REQUEST_ID_HTTP_HEADER_NAME;
use super::messages::{Message, SendMessage};
use super::response::MessageAPIResponse;
use futures::stream::Concat2;
use futures::{Future, Stream};
use reqwest::r#async::{Client as ReqwestClient, Decoder, Response};
use reqwest::{Error, StatusCode};
use serde::Deserialize;
use std::io::{self, Cursor};
use std::mem;

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

// fn parse_response<T>(mut res: Response) -> impl Future<Item = T, Error = Error> {
//     match res.status() {
//         StatusCode::OK => {}
//         _ => {}
//     };
//     if res.status() != StatusCode::OK {}
//     println!("{}", res.status());
//     let body = mem::replace(res.body_mut(), Decoder::empty());
//     body.concat2()
// }

impl Client {
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
    ) -> impl Future<Item = MessageAPIResponse, Error = ()> {
        let body = SendMessage::new(to, messages);
        let future = self
            .http_client
            .post(&format!("{}/post", self.host))
            .json(&body)
            .send()
            .and_then(|res| {
                let request_id = match res.headers().get(LINE_REQUEST_ID_HTTP_HEADER_NAME) {
                    Some(header_val) => match header_val.to_str() {
                        Ok(val) => Some(val.to_string()),
                        _ => None,
                    },
                    None => None,
                };
                Ok(MessageAPIResponse::new(request_id))
            })
            .map_err(|err| println!("request error: {}", err));
        // .map_err(|err| println!("request error: {}", err))
        // .map(|body| {
        //     let mut body = Cursor::new(body);
        //     let _ = io::copy(&mut body, &mut io::stdout()).map_err(|err| {
        //         println!("stdout error: {}", err);
        //     });
        // });
        future
    }
}

#[cfg(test)]
mod tests {
    extern crate futures;
    extern crate tokio;
    use super::super::messages::TextMessage;
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
