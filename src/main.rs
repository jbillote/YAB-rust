use regex::Regex;
use serde::Deserialize;
use serenity::{
    all::{
        Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
        GatewayIntents,
    },
    async_trait,
    model::{application::Command, application::Interaction, channel::Message, gateway::Ready},
    Client,
};
use std::{env, fs, process::exit, thread, time};
use toml;
use tracing::{error, info};
use tracing_subscriber::fmt;

mod commands;
mod models;
mod pixiv;
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
    pub statuses: Vec<String>,
    pub status_interval: i64,
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
                    info!("Cannot respond to command: {why}");
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let split_message = msg.content.split(" ");
        let mut peekable = split_message.clone().peekable();
        let mut supress_quote = peekable.peek().is_some_and(|&v| v == ".nq");
        let mut spoiler = false;
        for (ndx, m) in split_message.enumerate() {
            // TODO: Add more robust spoiler checks, i.e. check for closed ||
            spoiler = if &m[0..2] == "||" { true } else { spoiler };

            let twitter_regex =
                Regex::new(r"(\bx|\btwitter)\.com\/\w{1,15}\/(status|statuses)\/\d{2,20}").unwrap();
            let pixiv_regex = Regex::new(r"\bpixiv\.net\/\w{2}\/artworks\/\d{2,20}").unwrap();
            if twitter_regex.is_match(m) {
                info!("Twitter link found");
                let url = twitter_regex.find(m).unwrap();
                twitter::twitter::process_twitter_url(
                    &ctx,
                    &msg,
                    url.as_str(),
                    spoiler,
                    supress_quote,
                )
                .await;
            } else if pixiv_regex.is_match(m) {
                info!("Pixiv link found");
                let id = pixiv_regex
                    .find(m)
                    .unwrap()
                    .as_str()
                    .split("/")
                    .last()
                    .unwrap();
                pixiv::process_pixiv_id(&ctx, &msg, &id, spoiler).await;
            }
            peekable.next();
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let mbtl_command =
            Command::create_global_command(&ctx.http, commands::mbtl::register()).await;
        info!("Added slash command {}", mbtl_command.unwrap().name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = env::args().collect();
    let config_path = &args[1];
    let config_file = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => {
            error!("Could not read file {}", config_path);
            exit(1);
        }
    };
    let config: Config = match toml::from_str(&config_file) {
        Ok(c) => c,
        Err(_) => {
            error!("Unable to read data from {}", config_path);
            exit(1);
        }
    };
    let token = config.discord.token;
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
        exit(1);
    }
}
