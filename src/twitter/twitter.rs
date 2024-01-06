use crate::models::fxtwitter::FXTwitter;

use regex::Regex;
use reqwest::{header, header::HeaderMap, header::HeaderValue, Error};
use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    client::Context,
    model::{channel::Message, colour::Color, timestamp::Timestamp},
};
use tracing::{error, info};

pub async fn generate_twitter_embed(ctx: &Context, msg: &Message, url: &str) {
    let trim_regex = Regex::new(r"\w{1,15}\/(status|statuses)\/\d{2,20}").unwrap();
    let uri = trim_regex.find(url).unwrap();
    let info = get_tweet_info(uri.as_str()).await;

    let author = format!(
        "{user_name} (@{screen_name})",
        user_name = info.tweet.author.user_name,
        screen_name = info.tweet.author.screen_name
    );

    let mut embeds: Vec<CreateEmbed> = Vec::new();
    let mut videos: Vec<String> = Vec::new();

    embeds.push(
        CreateEmbed::new()
            .title("Original Tweet")
            .url(&info.tweet.url)
            .author(
                CreateEmbedAuthor::new(&author)
                    .icon_url(&info.tweet.author.avatar_url)
                    .url(&info.tweet.author.url),
            )
            .description(&info.tweet.text)
            .color(Color::BLUE)
            .footer(
                CreateEmbedFooter::new("Twitter")
                    .icon_url("https://abs.twimg.com/icons/apple-touch-icon-192x192.png"),
            )
            .timestamp(Timestamp::from_unix_timestamp(info.tweet.timestamp).unwrap()),
    );

    if let Some(media) = info.tweet.media {
        for link in media.media {
            match link.kind.as_str() {
                "gif" => videos.push(link.url),
                "photo" => embeds.push(CreateEmbed::new().url(&info.tweet.url).image(link.url)),
                "video" => videos.push(link.url),
                _ => info!("Unknown type: {}", link.kind),
            }
        }
    }

    let builder = CreateMessage::new().embeds(embeds);
    let res = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = res {
        error!("Error sending message: {why:?}");
    }

    for link in videos {
        if let Err(why) = msg.channel_id.say(&ctx.http, link).await {
            error!("Error sending message: {why:?}");
        }
    }

    if let Some(quote) = &info.tweet.quote {
        info!("Quote tweet found: {}", quote.url);

        let quote_author = format!(
            "{user_name} (@{screen_name})",
            user_name = quote.author.user_name,
            screen_name = quote.author.screen_name
        );

        let mut quote_embeds: Vec<CreateEmbed> = Vec::new();
        let mut quote_videos: Vec<String> = Vec::new();

        quote_embeds.push(
            CreateEmbed::new()
                .title("Quoted Tweet")
                .url(&quote.url)
                .author(
                    CreateEmbedAuthor::new(quote_author)
                        .icon_url(&quote.author.avatar_url)
                        .url(&quote.author.url),
                )
                .description(&quote.text)
                .color(Color::BLUE)
                .footer(
                    CreateEmbedFooter::new("Twitter")
                        .icon_url("https://abs.twimg.com/icons/apple-touch-icon-192x192.png"),
                )
                .timestamp(Timestamp::from_unix_timestamp(quote.timestamp).unwrap()),
        );

        if let Some(media) = &quote.media {
            for link in media.media.clone() {
                match link.kind.as_str() {
                    "gif" => quote_videos.push(link.url),
                    "photo" => {
                        quote_embeds.push(CreateEmbed::new().url(&quote.url).image(link.url))
                    }
                    "video" => quote_videos.push(link.url),
                    _ => info!("Unknown type: {}", link.kind),
                }
            }
        }

        let quote_builder = CreateMessage::new().embeds(quote_embeds);
        let quote_res = msg.channel_id.send_message(&ctx.http, quote_builder).await;

        if let Err(why) = quote_res {
            error!("Error sending message: {why:?}");
        }

        for link in quote_videos {
            if let Err(why) = msg.channel_id.say(&ctx.http, link).await {
                error!("Error sending message: {why:?}");
            }
        }
    }
}

async fn get_tweet_info(path: &str) -> FXTwitter {
    let url = format!("https://api.fxtwitter.com/{path}", path = path);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/116.0",
        ),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    info!("Calling fxtwitter API: {}", &url);
    let response = client.get(url).send().await.unwrap();
    let info: FXTwitter = response.json().await.unwrap();

    return info;
}
