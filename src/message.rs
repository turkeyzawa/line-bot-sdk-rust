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
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    r#type: &'static str,
    text: &'static str,
    notification_disabled: bool,
}

impl Message for TextMessage {}
impl TextMessage {
    pub fn new(text: &'static str, notification_disabled: bool) -> impl Message {
        Self {
            r#type: "text",
            text,
            notification_disabled,
        }
    }
}
