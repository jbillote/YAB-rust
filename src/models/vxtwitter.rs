use serde::Deserialize;

#[derive(Deserialize)]
pub struct VXTwitter {
    #[serde(alias = "date_epoch")]
    pub timestamp: i64,
    #[serde(alias = "mediaURLs")]
    pub media_urls: Vec<String>,
    pub text: String,
    #[serde(alias = "tweetURL")]
    pub tweet_url: String,
    #[serde(alias = "user_name")]
    pub user_name: String,
    #[serde(alias = "user_screen_name")]
    pub screen_name: String,
    pub user_profile_image_url: String,
}
