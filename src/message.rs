use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SendMessage<T: Message> {
    to: &'static str,
    messages: Vec<T>,
}

impl<T> SendMessage<T>
where
    T: Message,
{
    pub fn new(to: &'static str, messages: Vec<T>) -> Self {
        Self { to, messages }
    }
}

pub trait Message: Serialize {}

#[derive(Clone, Serialize, Deserialize)]
pub struct TextMessage {
    r#type: &'static str,
    text: &'static str,
}

impl Message for TextMessage {}
impl TextMessage {
    pub fn new(text: &'static str) -> impl Message {
        Self {
            r#type: "text",
            text: text,
        }
    }
}
