use anyhow::anyhow;
use regex::Regex;
use serde::Deserialize;
use serenity::{
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::{Client, Context, EventHandler},
    model::{
        application::{Command, Interaction},
        channel::Message,
        gateway::{GatewayIntents, Ready},
    },
};
use shuttle_secrets::SecretStore;
use tracing::{error, info};

mod commands;
mod models;
mod twitter;

struct Handler;

#[derive(Deserialize)]
struct Config {
    discord: Discord,
    frame_data: FrameData,
}

#[derive(Deserialize)]
struct Discord {
    pub token: String,
    pub status: String,
}

#[derive(Deserialize)]
struct FrameData {
    pub host: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!("Received command: {}", command.data.name.as_str());

            if command.data.name.as_str() == "mbtl" {
                let embed = commands::mbtl::run(&command.data.options()).await;
                let data = CreateInteractionResponseMessage::new().add_embed(embed);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("Cannot respond to command: {why}");
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let split_message = msg.content.split(" ");
        for m in split_message {
            let twitter_regex =
                Regex::new(r"(\bx|\btwitter)\.com\/\w{1,15}\/(status|statuses)\/\d{2,20}").unwrap();
            if twitter_regex.is_match(m) {
                let url = twitter_regex.find(m).unwrap();
                info!("Twitter link found: {}", url.as_str());
                twitter::twitter::generate_twitter_embed(&ctx, &msg, url.as_str()).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let mbtl_command =
            Command::create_global_command(&ctx.http, commands::mbtl::register()).await;
        info!("Added slash command {}", mbtl_command.unwrap().name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("DISCORD_TOKEN was not found").into());
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    Ok(client.into())
}
