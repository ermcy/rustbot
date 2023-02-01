use std::collections::HashSet;
use std::time::Instant;
use serenity::framework::standard::macros::{command, help};
use serenity::framework::standard::{Args, CommandGroup, CommandResult, help_commands, HelpOptions};
use serenity::model::prelude::*;
use serenity::prelude::*;


#[command]
#[description("Shows avatar of a user. If none is provided defaults to yourself.")]
#[usage("@user")]
#[aliases("av", "pfp")]
pub async fn avatar(ctx: &Context, message: &Message, _args: Args) -> CommandResult {
    let (embed_title, embed_image_url) = if let Some(mention) = message.mentions.first() {
        (mention.name.to_string(), mention.avatar_url().unwrap())
    } else { (message.author.name.to_string(), message.author.avatar_url().unwrap()) };
    message.channel_id.send_message(&ctx.http, |m| {
        m.reference_message(message);
        m.embed(|e| {
            e.title(format!("{}'s avatar", embed_title));
            e.timestamp(message.timestamp);
            e.image(embed_image_url);
            e
        })
    }).await?;
    Ok(())
}
#[command]
#[usage("ping")]
#[description("Ping Pong! Get the bot's websocket latency.")]
#[required_permissions("Administrator")]
pub async fn ping(ctx: &Context, message: &Message) -> CommandResult {
    let old = Instant::now();
    let mut last_message = message.reply_ping(&ctx.http, "Ping").await?;
    let new = Instant::now();
    let ping = (new - old).as_millis();
    let ping_message = format!("Pong! {} ms", ping);
    last_message.edit(&ctx.http, |m| {
        m.content(ping_message);
        m
    }).await?;
    Ok(())
}
#[help]
#[ungrouped_label("Ungrouped Commands")]
#[grouped_label("Group: ")]
#[lacking_ownership("hide")]
#[lacking_permissions("hide")]
#[embed_error_colour("#FF0000")]
#[embed_success_colour("#00FF00")]
pub async fn help(ctx: &Context,
                  message: &Message,
                  args: Args,
                  help_options: &HelpOptions,
                  group: &[&'static CommandGroup],
                  owners: HashSet<UserId>) -> CommandResult {
    help_commands::with_embeds(ctx, message, args, help_options, group, owners).await?;
    Ok(())
}
