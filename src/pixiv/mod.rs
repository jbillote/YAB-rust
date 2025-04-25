use crate::models::phixiv::Phixiv;

use regex::Regex;
use reqwest::Client;
use serenity::{
    builder::{
        CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage,
    },
    client::Context,
    model::{channel::Message, colour::Color, timestamp::Timestamp},
};
use std::{str::FromStr, thread, time};
use tracing::{error, info};

pub async fn process_pixiv_id(ctx: &Context, msg: &Message, id: &str, spoiler: bool) {
    if spoiler {
        info!("Pixiv image sent as spoiler, do not generate embeds");
        return;
    }

    let info = get_pixiv_info(id).await;
    let embeds = generate_pixiv_embed(&info).await;

    let builder = CreateMessage::new()
        .embeds(embeds)
        .reference_message(msg)
        .allowed_mentions(CreateAllowedMentions::new().replied_user(false));
    let res = msg.channel_id.send_message(&ctx.http, builder).await;

    if let Err(why) = res {
        error!("Error sending message: {why:?}");
    }
}

async fn get_pixiv_info(id: &str) -> Phixiv {
    let url = format!("https://www.phixiv.net/api/info?id={id}", id = id);
    let client = reqwest::Client::builder().build().unwrap();
    info!("Calling Phixiv API: {}", &url);
    let response = client.get(url).send().await.unwrap();
    let info: Phixiv = response.json().await.unwrap();

    return info;
}

async fn generate_pixiv_embed(pixiv: &Phixiv) -> Vec<CreateEmbed> {
    let mut embeds: Vec<CreateEmbed> = Vec::new();

    for link in &pixiv.images {
        if embeds.is_empty() {
            embeds.push(
                CreateEmbed::new()
                    .title(&pixiv.title)
                    .url(&pixiv.url)
                    .author(
                        CreateEmbedAuthor::new(&pixiv.author)
                            .icon_url(&pixiv.profile_image_url)
                            .url(format!("https://www.pixiv.net/users/{}", &pixiv.author_id)),
                    )
                    .description(strip_html(&pixiv.description))
                    .image(link)
                    .color(Color::BLUE)
                    .footer(CreateEmbedFooter::new("Pixiv"))
                    .timestamp(Timestamp::from_str(&pixiv.create_date).unwrap()),
            );
        } else {
            embeds.push(CreateEmbed::new().url(&pixiv.url).image(link));
        }
    }

    return embeds;
}

fn strip_html(text: &str) -> String {
    let re = Regex::new(r#"<a.*">"#).unwrap();
    let stripped_string = re.replace_all(text, "");
    return stripped_string.replace("<br />", "\n").replace("</a>", "");
}
