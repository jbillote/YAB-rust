# YAB
## Yet Another (Discord) Bot

Basic Discord chat bot written in Rust. As of now, its primary function is to 
take Twitter links and re-create the, as of now, broken Twitter embeds. Some 
improvements have been made over the original embeds, such as multi-image 
embeds working on mobile.

## Hosting/Deployment

YAB is designed to be run on bare-metal. A sample ``config.toml`` file that 
contains required keys can be found as ``config.toml``. You can specify which 
config file to use when running YAB by specifying it as an argument ie:

```
cargo run /path/to/config
```

## Known Issues

* Videos from Twitter are not included in the embed. This appears to be a 
problem on Discord's end, as the issue seems to be present in other languages 
and libraries.
