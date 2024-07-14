use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SendMessageRequest {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ReplyMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
    },
    ReplyKeyboardMarkup {
        keyboard: Vec<Vec<KeyboardButton>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        resize_keyboard: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        one_time_keyboard: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
    ReplyKeyboardRemove {
        remove_keyboard: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
    ForceReply {
        force_reply: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub poll_type: Option<String>,
}
