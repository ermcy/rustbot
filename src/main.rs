use std::{env};
use std::collections::{HashSet};
use serenity::async_trait;
use serenity::framework::standard::macros::{ group };
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

// here we add commands from the folder
mod commands;

use crate::commands::general::*;

// structs
struct Handler;

// struct implementations
#[async_trait]
impl EventHandler for Handler {
    // ready for when bot is ready
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("{}#{} is ready!", _data_about_bot.user.name, _data_about_bot.user.discriminator);
    }
}
#[group]
#[commands(avatar, ping)]
struct General;

// main function
#[tokio::main]
async fn main() {
    let bot_token = env::var("DISCORD_TOKEN")
        .expect("No discord token was provided");
    let bot_intents = GatewayIntents::all();
    let http = Http::new(&bot_token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else { owners.insert(info.owner.id); }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access bot id {:?}", why)
            }
        }
        Err(why) => panic!("Could not access application info {:?}", why)
    };
    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("~")
            .delimiters(vec![", ", ","])
            .owners(owners)
        )
        .help(&HELP)
        .group(&GENERAL_GROUP);
    let mut client = Client::builder(&bot_token, bot_intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("There was an error.");
    if let Err(why) = client.start().await {
        println!("Error {:?}", why)
    }
}
