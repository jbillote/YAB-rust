# YAB
## Yet Another (Discord) Bot

Basic Discord chat bot written in Rust. As of now, its primary function is to 
take Twitter links and re-create the, as of now, broken Twitter embeds. Some 
improvements have been made over the original embeds, such as multi-image 
embeds working on mobile.

## Hosting/Deployment

YAB is designed to be deployed on Shuttle. A sample ``Secrets.toml`` file that 
contains required keys can be found as ``Secrets.dev.toml.template``. As of
now, the only required key is the Discord bot token.

## Known Issues

* Videos from Twitter are not included in the embed. This appears to be a 
problem on Discord's end, as the issue seems to be present in other languages 
and libraries.
* Tweet text is sometimes cut off, despite the character limit not being 
reached. This is in part due to the API used to fetch tweet information, but 
the root cause is being investigated.
