use serenity::builder::{CreateEmbed, CreateMessage, EditMessage};
use serenity::model::colour::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::time::{sleep, Duration};

pub async fn show_loading_animation(
    ctx: &Context,
    msg: &Message,
    initial_message: &str,
) -> Message {
    let mut embed = CreateEmbed::default()
        .description(initial_message)
        .color(Colour::DARK_BLUE);

    let mut message = CreateMessage::default().embed(embed.clone());

    // Send initial message
    let mut loading_msg = msg
        .channel_id
        .send_message(&ctx.http, message)
        .await
        .unwrap();

    // Create a loading animation
    let loading_states = ["Loading.", "Loading..", "Loading...", "Loading...."];
    for state in &loading_states {
        embed = embed.description(*state);
        let edit_message = EditMessage::new().embed(embed.clone());

        loading_msg.edit(&ctx.http, edit_message).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }

    loading_msg
}
