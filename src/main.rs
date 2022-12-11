use chat_gpt_slack::prelude::*;

const YOUR_SESSION_TOKEN: &'static str = "";

fn main() {
    let session = SessionToken::new(YOUR_SESSION_TOKEN);
    let client = ChatGPT::new(session);

    let response = client.ask("Hello").unwrap();

    println!("Response: {:?}", response);
}
