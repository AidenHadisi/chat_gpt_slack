use reqwest::{
    blocking::Client,
    header,
    header::{HeaderMap, HeaderValue},
};

use super::auth::SessionToken;
use super::request::Request;
use super::Response;
use crate::Error::ApiError;
use crate::Result;

///ChatGPTApi provides a trait for api clients that can interact with ChatGPT.
pub trait ChatGPTApi {
    ///Asks a question from ChatGPT and returns response.
    fn ask(self, question: &str) -> Result<Response>;
}

///Default ChatGPT api implementation.
pub struct ChatGPT {
    ///Http client for making requests.
    client: Client,
}

impl ChatGPT {
    const API_BASE: &'static str = "https://chat.openai.com/backend-api/conversation";
    const CONTENT_TYPE: &'static str = "application/json";
    const ACCEPT: &'static str = "text/event-stream";
    const REFERER: &'static str = "https://chat.openai.com/chat";
    const ORIGIN: &'static str = "https://chat.openai.com";
    ///A valid user agent is needed to get a response.
    const USER_AGENT: &'static str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.1 Safari/605.1.15";
    const ASSISTANT_APP_ID: &'static str = "";

    ///Creates a new instance of default ChatGPT api client.
    pub fn new(session: SessionToken) -> Self {
        let headers = Self::create_default_headers(session);
        let c = Client::builder().default_headers(headers);

        Self {
            client: c.build().unwrap(),
        }
    }

    fn create_default_headers(session: SessionToken) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(Self::CONTENT_TYPE),
        );
        headers.insert(header::ACCEPT, HeaderValue::from_static(Self::ACCEPT));
        headers.insert(header::REFERER, HeaderValue::from_static(Self::REFERER));
        headers.insert(header::ORIGIN, HeaderValue::from_static(Self::ORIGIN));
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(Self::USER_AGENT),
        );
        headers.insert(
            "X-OpenAI-Assistant-App-Id",
            HeaderValue::from_static(Self::ASSISTANT_APP_ID),
        );

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(session.bearer().as_str()).unwrap(),
        );

        headers
    }
}

impl ChatGPTApi for ChatGPT {
    fn ask(self, question: &str) -> Result<Response> {
        let request = Request::new(question.to_string());

        let result = self
            .client
            .post(Self::API_BASE)
            .json(&request)
            .send()
            .map_err(|e| ApiError("got an error from ChatGPT api. try again later".to_owned()))?;

        let text = result
            .text()
            .map_err(|e| ApiError("failed to get result text from response".to_owned()))?;

        let messages: Vec<Response> = text
            .lines()
            .filter_map(|l| serde_json::from_str(l.replace("data: ", "").as_str()).ok())
            .collect();

        let last_message = messages.last();

        match last_message {
            Some(message) => Ok(message.clone()),
            None => Err(ApiError(
                "ChatGPT api did not return any messages :(".to_owned(),
            )),
        }
    }
}
