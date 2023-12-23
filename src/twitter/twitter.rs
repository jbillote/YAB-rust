use crate::models::vxtwitter::VXTwitter;

use regex::Regex;
use reqwest::Error;
use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    client::Context,
    model::{channel::Message, timestamp::Timestamp},
};

pub async fn generate_twitter_embed(ctx: &Context, msg: &Message, url: &str) {
    let trim_regex = Regex::new(r"\w{1,15}\/(status|statuses)\/\d{2,20}").unwrap();
    let uri = trim_regex.find(url).unwrap();
    let info = get_tweet_info(uri.as_str()).await;

    let author = format!(
        "{user_name} (@{screen_name})",
        user_name = info.user_name,
        screen_name = info.screen_name
    );
    let author_url = format!(
        "https://twitter.com/{screen_name}",
        screen_name = info.screen_name
    );
    let split_description: Vec<&str> = info.text.split(" ").collect();

    let mut embeds: Vec<CreateEmbed> = Vec::new();
    let mut videos: Vec<String> = Vec::new();

    for link in info.media_urls {
        let split_url: Vec<&str> = link.split(".").collect();
        let extension = split_url.last().unwrap();

        if extension.contains("jpg") || extension.contains("jpeg") || extension.contains("png") {
            if embeds.is_empty() {
                embeds.push(
                    CreateEmbed::new()
                        .title("Original Tweet")
                        .url(&info.tweet_url)
                        .author(
                            CreateEmbedAuthor::new(&author)
                                .icon_url(&info.user_profile_image_url)
                                .url(&author_url),
                        )
                        .description(info.text.replace(split_description.last().unwrap(), ""))
                        .footer(
                            CreateEmbedFooter::new("Twitter")
                                .icon_url("http://i.toukat.moe/twitter_logo.png"),
                        )
                        .timestamp(Timestamp::from_unix_timestamp(info.timestamp).unwrap())
                        .image(link),
                );
            } else {
                embeds.push(CreateEmbed::new().url(&info.tweet_url).image(link));
            }
        } else if extension.contains("mp4") {
            videos.push(link);
        }
    }

    if embeds.is_empty() {
        embeds.push(
            CreateEmbed::new()
                .title("Original Tweet")
                .url(&info.tweet_url)
                .author(
                    CreateEmbedAuthor::new(&author)
                        .icon_url(&info.user_profile_image_url)
                        .url(&author_url),
                )
                .description(info.text.replace(split_description.last().unwrap(), ""))
                .footer(
                    CreateEmbedFooter::new("Twitter")
                        .icon_url("http://i.toukat.moe/twitter_logo.png"),
                )
                .timestamp(Timestamp::from_unix_timestamp(info.timestamp).unwrap()),
        );
    }

    let builder = CreateMessage::new().embeds(embeds);
    let res = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = res {
        println!("Error sending message: {why:?}");
    }

    for link in videos {
        if let Err(why) = msg.channel_id.say(&ctx.http, link).await {
            println!("Error sending message: {why:?}");
        }
    }
}

async fn get_tweet_info(path: &str) -> VXTwitter {
    let url = format!("https://api.vxtwitter.com/{path}", path = path);
    let response = reqwest::get(&url).await.unwrap();
    let info: VXTwitter = response.json().await.unwrap();

    return info;
}
