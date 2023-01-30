# rusty_discord
Discord Bot using Serenity and Songbird Crate.

[Serenity] is a Rust library for the Discord API

[Songbird] is an async, cross-library compatible voice system for Discord, written in Rust
For using Songbird and streaming music folllowing dependencies need to be meet:
- Opus - Audio codec that Discord uses.
- FFmpeg - Audio/Video conversion tool.
- youtube-dl - Audio/Video download tool.


Cargo.toml:
```toml
[dependencies]
tokio = { version = "1.18", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
toml = "0.5.9"
serde = "1.0.137"
serde_derive = "1.0.137"
serde_json = "1.0.81"
songbird = "0.3.0"

[dependencies.serenity]
default-features = false
features = [
    "builder",
    "chrono",
    "client",
    "framework",
    "gateway",
    "http",
    "model",
    "standard_framework",
    "utils",
    "rustls_backend",
    "voice",
    "cache",
]
version ="0.11"
````

Place Your Discord Bot Token in config.toml:
```toml
token = "YOUR_DISCORD_BOT_TOKEN"
````
[MemeAPI] is used for `meme` and `gif` command.
To get All command type `Ru help` in server.


[Serenity]: https://github.com/serenity-rs/serenity
[Songbird]: https://github.com/serenity-rs/songbird
[MemeAPI]: https://meme-api.com/gimme
