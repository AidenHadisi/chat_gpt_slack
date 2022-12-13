use dotenv;
use std::{env, sync::Arc};

use chat_gpt_slack::prelude::*;
use slack_morphism::prelude::*;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    env::var("CHAT_GPT_SESSION_TOKEN").expect("missing session token");
    let slack_token = env::var("SLACK_APP_TOKEN").expect("missing slack token");

    create_slack_listener(slack_token.as_str()).await;
    return
}

async fn create_slack_listener(token: &str) {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let socket_mode_callbacks =
        SlackSocketModeListenerCallbacks::new().with_command_events(handle_command);

    let listener_environment = Arc::new(SlackClientEventsListenerEnvironment::new(client.clone()));

    let socket_mode_listener = SlackClientSocketModeListener::new(
        &SlackClientSocketModeConfig::new(),
        listener_environment.clone(),
        socket_mode_callbacks,
    );
    let app_token_value: SlackApiTokenValue = token.into();
    let app_token: SlackApiToken = SlackApiToken::new(app_token_value);
    socket_mode_listener.listen_for(&app_token).await.unwrap();
    socket_mode_listener.serve().await;
}

async fn handle_command(
    e: SlackCommandEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    let Some(text) = e.text else {
        return Ok(SlackCommandEventResponse::new(
            SlackMessageContent::new().with_text("Sorry something went wrong when trying to parse your command.".into()),
        ))
    };

    let token = env::var("CHAT_GPT_SESSION_TOKEN").expect("missing slack token");

    let session = SessionToken::new(token);
    let chat_gpt_api = ChatGPT::new(session);

    let res = chat_gpt_api.ask(&text).await;

    let message = match res {
        Ok(message) => message.message.content.parts[0].clone(),
        Err(e) => e.to_string(),
    };

    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text(message.into()),
    ))
}
