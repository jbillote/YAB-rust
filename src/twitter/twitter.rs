use crate::models::fxtwitter::{FXTwitter, Tweet};

use regex::Regex;
use reqwest::{header, header::HeaderMap, header::HeaderValue, Error};
use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    client::Context,
    model::{channel::Message, colour::Color, timestamp::Timestamp},
};
use tracing::{error, info};

pub async fn process_twitter_url(ctx: &Context, msg: &Message, url: &str, supress_quote: bool) {
    let trim_regex = Regex::new(r"\w{1,15}\/(status|statuses)\/\d{2,20}").unwrap();
    let uri = trim_regex.find(url).unwrap();
    let info = get_tweet_info(uri.as_str()).await;

    let main_tweet = generate_tweet_embeds(&info.tweet, false).await;

    let builder = CreateMessage::new().embeds(main_tweet.0);
    let res = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = res {
        error!("Error sending message: {why:?}");
    }

    for link in main_tweet.1 {
        if let Err(why) = msg.channel_id.say(&ctx.http, link).await {
            error!("Error sending message: {why:?}");
        }
    }

    if !supress_quote {
        if let Some(quote) = &info.tweet.quote {
            info!("Quote tweet found: {}", quote.url);

            let quote_tweet = generate_tweet_embeds(quote, true).await;
            let quote_builder = CreateMessage::new().embeds(quote_tweet.0);
            let quote_res = msg.channel_id.send_message(&ctx.http, quote_builder).await;

            if let Err(why) = quote_res {
                error!("Error sending message: {why:?}");
            }

            for link in quote_tweet.1 {
                if let Err(why) = msg.channel_id.say(&ctx.http, link).await {
                    error!("Error sending message: {why:?}");
                }
            }
        }
    }
}

async fn generate_tweet_embeds(tweet: &Tweet, is_quote: bool) -> (Vec<CreateEmbed>, Vec<String>) {
    let author = format!(
        "{user_name} (@{screen_name})",
        user_name = tweet.author.user_name,
        screen_name = tweet.author.screen_name
    );

    let mut embeds: Vec<CreateEmbed> = Vec::new();
    let mut videos: Vec<String> = Vec::new();

    

    if let Some(media) = &tweet.media {
        for link in media.media.clone() {
            match link.kind.as_str() {
                "gif" => videos.push(link.url),
                "photo" => {
                    if embeds.is_empty() {
                        embeds.push(
                            CreateEmbed::new()
                                .title(if is_quote {
                                    "Quoted Tweet"
                                } else {
                                    "Original Tweet"
                                })
                                .url(&tweet.url)
                                .author(
                                    CreateEmbedAuthor::new(&author)
                                        .icon_url(&tweet.author.avatar_url)
                                        .url(&tweet.author.url),
                                )
                                .description(&tweet.text)
                                .image(link.url)
                                .color(Color::BLUE)
                                .footer(
                                    CreateEmbedFooter::new("Twitter")
                                        .icon_url("https://abs.twimg.com/icons/apple-touch-icon-192x192.png"),
                                )
                                .timestamp(Timestamp::from_unix_timestamp(tweet.timestamp).unwrap()),
                        );
                    } else {
                        embeds.push(CreateEmbed::new().url(&tweet.url).image(link.url))
                    }
                },
                "video" => videos.push(link.url),
                _ => info!("Unknown type: {}", link.kind),
            }
        }
    }

    if embeds.is_empty() {
        embeds.push(
            CreateEmbed::new()
                .title(if is_quote {
                    "Quoted Tweet"
                } else {
                    "Original Tweet"
                })
                .url(&tweet.url)
                .author(
                    CreateEmbedAuthor::new(&author)
                        .icon_url(&tweet.author.avatar_url)
                        .url(&tweet.author.url),
                )
                .description(&tweet.text)
                .color(Color::BLUE)
                .footer(
                    CreateEmbedFooter::new("Twitter")
                        .icon_url("https://abs.twimg.com/icons/apple-touch-icon-192x192.png"),
                )
                .timestamp(Timestamp::from_unix_timestamp(tweet.timestamp).unwrap()),
        );
    }

    return (embeds, videos);
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
