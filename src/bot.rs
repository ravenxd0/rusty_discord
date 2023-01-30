use std::fs;

use std::collections::HashSet;

use serde_derive::Deserialize;
use serde_json::Value;

use reqwest::get;

use serenity::prelude::*;
use serenity::async_trait;
use serenity::framework::standard::{
    CommandResult,
    CommandGroup,
    StandardFramework,
    Args,
    HelpOptions,
    help_commands,
    macros::{command, group, help }
};
use serenity::model::{
    prelude::Member,
    id::UserId,
    channel::Message,
    user::OnlineStatus,
    gateway::{Ready,Activity},
};
use serenity::utils::Color;
use serenity::Result as SerenityResult;

use songbird::SerenityInit;

#[derive(Deserialize)]
struct Bot {
    token: String,
}

impl Bot {
    fn new() -> Self {
        let content = fs::read_to_string("config.toml").unwrap();
        toml::from_str(&content).unwrap()// Create Bot Struct using toml values
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Runs on Connection to discord and prints all the server bot is connected to
    async fn ready(&self, ctx: Context, msg: Ready) {
        println!("[ READY ] {} is connected.", msg.user.name);

        if let Ok(guilds) = msg.user.guilds(&ctx.http).await {
            for guild in guilds.into_iter() {
                println!("\t- {}", guild.name);
            }
        }

        // Set Presence of bot
        ctx.set_presence(
            Some(Activity::listening("Ru help")),
            OnlineStatus::Idle,
            ).await
    }

    // Dispatched when  message is created 
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content.to_lowercase().starts_with("hello ru") {
            msg.channel_id.broadcast_typing(&ctx).await.unwrap();

            handle(
            msg.reply(ctx, format!("@97723693Hello <@{}>",msg.author.id)).await
                );

        }
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Some(channel) = new_member.default_channel(&ctx.cache) {

           handle(
            channel.id.send_message(ctx, |m| {
                m.content( format!("<@{}> Joined in.", new_member.user.id) )
            }).await);

        };
    }
}

#[group]
#[commands(ping, meme, gif, details)]
struct General;

#[command]
#[description = "Reply With Pong!"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;

    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
#[description = "Reply With random meme image"]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;


    let response =  get("https://meme-api.com/gimme").await?.text().await?;
    let response: Value = serde_json::from_str(response.as_str())?;
    let url =format!("{}",response["preview"][3]).replace("\"","");

    msg.reply(ctx, url).await?;

    Ok(())
}

#[command]
#[description = "Reply with random meme gif"]
async fn gif(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;


    let response =  get("https://meme-api.com/gimme").await?.text().await?;
    let response: Value = serde_json::from_str(response.as_str())?;
    let title =format!("{}",response["title"]).replace("\"","");
    let url =format!("{}",response["url"]).replace("\"","");

    msg.channel_id.send_message(ctx, |m| {
        m.embed( |e| 
            e.title(title)
            .image(url)
            .color(Color::DARK_GREY)
        )
    }).await?; 

    Ok(())

}

#[command]
#[description = "Sends Server's Information"]
#[only_in(guilds)]
#[allowed_roles("Admin")]
async fn details(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;


    let server = msg.guild(&ctx.cache).unwrap();
    let server_name = &server.name;
    let thumbnail = &server.icon_url().unwrap_or("No Icon".to_owned());
    let owner = server.owner_id.to_user(&ctx.http).await?;
    let members = server.members;
    let members_count = members.len();

    msg.channel_id.send_message(ctx, |m| {
        m.embed(|e| {
            e.title(format!("{} Server's Info:", server_name))
                .field("Owner",owner.name, false)
                .field("Server ID",server.id.0, false)
                .field("Member Count",members_count, false)
                .color(Color::FABLED_PINK)
                .thumbnail(thumbnail) 
        })
    }).await?;

    for member in members.into_values() {
        let content = format!("Member name: {}\nID: {}\nJoined at: {}",
            &member.user.name,
            &member.user.id,
            &member.joined_at.unwrap(),
            ).to_string();

        msg.channel_id.send_message(ctx, |m| m.content(content)).await?;
    }

    Ok(())
}

// Music 
#[group]
#[commands(join, leave, play, mute, unmute)]
struct Music;

#[command]
#[description = "Join Voice Channel"]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();

    let channel_id = guild.voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id );

    if let Some(channel) = channel_id {
        let manager = songbird::get(ctx).await
            .expect("Songbird Voice Client placed in at Initialization.");

        let _ = manager.join(guild.id, channel).await;

        msg.reply(ctx, "Joined Voice Channel.").await?;

    } else {
        msg.reply(ctx, "Not in a Voice Channel.").await?;
    }

    Ok(())
}

#[command]
#[description = "Leave Voice Channel"]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
   let guild = msg.guild(&ctx.cache).unwrap();

   let manager = songbird::get(ctx).await
        .expect("Songbird Voice Client placed in at intialization.");

    let has_handler = manager.get(guild.id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild.id).await {
            msg.channel_id.say(&ctx.http, format!("Failed: {:?}",e)).await?;
        } else {
            msg.channel_id.say(&ctx.http, "Left Voice Channel.").await?;
        }

    } else {
        msg.reply(ctx, "Not in a Voice Channel.").await?;
    }


    Ok(())
}

#[command]
#[description = "Play a audio using video or audio url"]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio.").await?;
            return Ok(());

        }
    };

    if !url.starts_with("http") {
        msg.channel_id.say(&ctx.http, "Must Provide a valid URL").await?;
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice Client placed in at Initialization.");

    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(e) => {
                eprintln!("[ ERROR ] {:?}",e);
                msg.channel_id.say(&ctx.http, "Error Sourcing FFMPEG").await?;

                return Ok(());
            }
        };

        let title = source.metadata.title.clone().unwrap_or("Unknown".to_string());
        handler.play_source(source); // Play audio om channel
        msg.channel_id.say(&ctx.http, format!("Playing {}", title) ).await?;
    } else {
        msg.channel_id.say(&ctx.http, "Not in a Voice channel to play in").await?;
    }

    Ok(())
}

#[command]
#[description = "Mute Bot in Voice channel"]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;

    let guild = msg.guild(&ctx.cache).unwrap();

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice Client placed in at intialization.");

    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;
        
        if handler.is_mute() {
            msg.reply(ctx, "Already muted.").await?;
        } else {
            if let Err(e) = handler.mute(true).await {
                msg.channel_id.say(&ctx.http, format!("Failed: {:?}",e)).await?;
            } else {
                msg.channel_id.say(&ctx.http, "Now muted.").await?;
            }

        }

    } else {
        msg.reply(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}


#[command]
#[description = "Unmute Bot in Voice channel"]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;
    
    let guild = msg.guild(&ctx.cache).unwrap();
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice Client placed in at intialization.");
 
    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;
        
        if let Err(e) = handler.mute(false).await {
            msg.channel_id.say(&ctx.http, format!("Failed: {:?}",e)).await?;
        } else {
            msg.channel_id.say(&ctx.http, "Unmuted").await?;
        }
        
    } else {
        msg.reply(ctx, "Not in a voice channel to unmute in").await?;
    }

    Ok(())
}

// Help Function
#[help]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
    ) -> CommandResult {
    msg.channel_id.broadcast_typing(&ctx).await?;

    let _ = help_commands::with_embeds(ctx,msg,args,options,groups,owners).await;
    Ok(())
}


// Client Initialization
pub async fn init_client() -> Client {
    let bot = Bot::new();

    let framework = StandardFramework::new()
        .configure(|c| 
            c.with_whitespace(true)
            .prefix("Ru ")
        )
        .help(&MY_HELP)
        .group(&GENERAL_GROUP)
        .group(&MUSIC_GROUP);

    let intents = GatewayIntents::all();

    Client::builder(bot.token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .unwrap()
}

// Check that a message succesfully sent, if not, log to stderr
fn handle(result: SerenityResult<Message>) {
    if let Err(why) = result {
        eprintln!("[ ERROR ] {:?}",why);
    }
}
