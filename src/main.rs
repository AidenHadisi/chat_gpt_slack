use chat_gpt_slack::chat_gpt::api::ChatGPTApi;

const YOUR_SESSION_TOKEN: &'static str = "";

fn main() {
    let session = chat_gpt_slack::chat_gpt::auth::SessionToken::new(YOUR_SESSION_TOKEN);
    let client = chat_gpt_slack::chat_gpt::api::ChatGPT::new(session);

    let response = client.ask("Hello").unwrap();

    println!("Response: {:?}", response);
}
