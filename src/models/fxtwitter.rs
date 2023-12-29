use serde::Deserialize;

#[derive(Deserialize)]
pub struct FXTwitter {
    pub tweet: Tweet,
}

#[derive(Deserialize)]
pub struct Tweet {
    pub url: String,
    pub text: String,
    pub author: Author,
    #[serde(alias = "created_timestamp")]
    pub timestamp: i64,
    #[serde(default)]
    pub media: Media,
}

#[derive(Deserialize)]
pub struct Author {
    #[serde(alias = "name")]
    pub user_name: String,
    pub screen_name: String,
    #[serde(alias = "avatar")]
    pub avatar_url: String,
    pub url: String,
}

#[derive(Deserialize, Default)]
pub struct Media {
    #[serde(alias = "all")]
    pub media: Vec<Attachment>,
}

#[derive(Deserialize)]
pub struct Attachment {
    pub url: String,
    #[serde(alias = "type")]
    pub kind: String,
}
