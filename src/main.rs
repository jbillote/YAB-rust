use serenity::{
    all::{
        Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
        GatewayIntents,
    },
    async_trait,
    model::{application::Command, application::Interaction, gateway::Ready},
    Client,
};
use std::env;

mod commands;
mod models;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command: {}", command.data.name.as_str());

            if command.data.name.as_str() == "mbtl" {
                let embed = commands::mbtl::run(&command.data.options()).await;
                let data = CreateInteractionResponseMessage::new().add_embed(embed);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to command: {why}");
                }
            }

            // let content = match command.data.name.as_str() {
            //     "mbtl" => Some(commands::mbtl::run(&command.data.options())),
            //     _ => todo!(),
            // };

            // if let Some(content) = content {
            //     let data = CreateInteractionResponseMessage::new().add_embed(content.await);
            //     let builder = CreateInteractionResponse::Message(data);
            //     if let Err(why) = command.create_response(&ctx.http, builder).await {
            //         println!("Cannot respond to command: {why}");
            //     }
            // };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let mbtl_command =
            Command::create_global_command(&ctx.http, commands::mbtl::register()).await;
        println!("Added slash command {}", mbtl_command.unwrap().name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
