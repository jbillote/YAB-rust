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
    #[serde(alias = "possibly_sensitive")]
    pub nsfw: Option<bool>,
    pub media: Option<Media>,
    pub quote: Option<Box<Tweet>>,
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

#[derive(Deserialize)]
pub struct Media {
    #[serde(alias = "all")]
    pub media: Vec<Attachment>,
    pub videos: Option<Vec<Attachment>>,
    pub photos: Option<Vec<Attachment>>,
}

#[derive(Deserialize, Clone)]
pub struct Attachment {
    pub url: String,
    #[serde(alias = "type")]
    pub kind: String,
}
