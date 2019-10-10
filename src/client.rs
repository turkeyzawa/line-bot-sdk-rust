extern crate futures;
extern crate reqwest;
use super::constants::LINE_REQUEST_ID_HTTP_HEADER_NAME;
use super::message::{Message, SendMessage};
use super::response::MessageAPIResponse;
use futures::prelude::*;
use reqwest::r#async::{Client as ReqwestClient, Response};
pub use reqwest::Error;
use std::env;

#[derive(Clone)]
pub struct ClientConfig {
    channel_access_token: &'static str,
}

impl ClientConfig {
    pub fn new(channel_access_token: &'static str) -> Self {
        Self {
            channel_access_token,
        }
    }
}

#[derive(Clone)]
pub struct Client {
    config: ClientConfig,
    http_client: ReqwestClient,
    host: String,
}

impl Client {
    fn parse_message_response(res: Response) -> Result<MessageAPIResponse, Error> {
        res.error_for_status().map(|res| {
            let request_id = match res.headers().get(LINE_REQUEST_ID_HTTP_HEADER_NAME) {
                Some(header_val) => match header_val.to_str() {
                    Ok(val) => Some(val.to_string()),
                    _ => None,
                },
                None => None,
            };
            MessageAPIResponse::new(request_id)
        })
    }

    pub fn new(config: ClientConfig) -> Self {
        let host = env::var("API_BASE_URL")
            .or::<env::VarError>(Ok(String::from("https://api.line.me/v2/bot/")))
            .unwrap();
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
            .post(&format!("{}/message/push", self.host))
            .json(&body)
            .send()
            .and_then(Client::parse_message_response)
    }
}
