use serde::Deserialize;

#[derive(Deserialize)]
pub struct Phixiv {
    #[serde(alias = "image_proxy_urls")]
    pub images: Vec<String>,
    pub title: String,
    pub ai_generated: bool,
    pub description: String,
    pub url: String,
    #[serde(alias = "author_name")]
    pub author: String,
    pub author_id: String,
    pub create_date: String,
    pub profile_image_url: String,
}
