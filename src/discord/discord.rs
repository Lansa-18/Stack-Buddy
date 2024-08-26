use reqwest::Client as ReqwestClient;
use serenity::async_trait;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage};
use serenity::model::colour::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::api::{
    get_featured_campaigns, get_hackathons, get_pathways, get_user_balance, get_user_details,
    get_user_progress,
};
use crate::discord::messages::{HELP_MESSAGE, INFO_MESSAGE};
use crate::util::show_loading_animation;

// Creating a function to display embedded messages
pub async fn send_embed_message(
    command_name: &str,
    message_details: &str,
    color: Colour,
    ctx: &Context,
    msg: &Message,
    image_url: Option<&str>,
) {
    // Check if the message content matches the command name
    if msg.content == command_name {
        // Create an author for the embed message
        let author = CreateEmbedAuthor::new("Stack-Buddy");

        // Initialize the embed message with author, description, thumbnail, timestamp, and color
        let mut embed = CreateEmbed::default()
            .author(author)
            .description(message_details)
            .thumbnail("https://i.imgur.com/xRT3jsZ.png")
            .timestamp(Timestamp::now())
            .color(color);

        // If an image URL is provided, add it to the embed
        if let Some(url) = image_url {
            embed = embed.image(url);
        }

        // Create the message with the embed
        let message = CreateMessage::default().embed(embed);

        if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

pub struct Handler {
    pub http_client: ReqwestClient,
    pub base_url: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Trim the message content to remove any leading/trailing whitespace
        let content = msg.content.trim();
        // Match the trimmed content to the corresponding command handler
        match content {
            "!get-username" => self.handle_get_username(&ctx, &msg).await,
            "!help" => self.handle_help(&ctx, &msg).await,
            "!info" => self.handle_info(&ctx, &msg).await,
            "!get-balance" => self.handle_get_balance(&ctx, &msg).await,
            "!get-profile" => self.handle_get_profile(&ctx, &msg).await,
            "!get-campaigns" => self.handle_get_campaigns(&ctx, &msg).await,
            "!get-pathways" => self.handle_get_pathways(&ctx, &msg).await,
            "!get-hackathons" => self.handle_get_hackathons(&ctx, &msg).await,
            "!get-calendar" => self.handle_get_calendar(&ctx, &msg).await,
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        // Print a message when the bot is connected
        println!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    async fn handle_get_username(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        let user_id: i32 = 1;
        // Check if the user ID is valid
        if user_id > 0 {
            // Fetch user details and handle the result
            match get_user_details(&self.http_client, &self.base_url, user_id).await {
                Ok(user) => {
                    // Format the user details into a message
                    let message_details = format!(
                        "ID: {}\nUsername: {}\nNationality: {}\nCareer Level: {}\nRole: {}\nTech Stacks: {}",
                        user.id, user.username, user.nationality, user.career_level, user.role, user.tech_stack
                    );
                    // Send the formatted message as an embed
                    send_embed_message(
                        "!get-username",
                        &message_details,
                        Colour::DARK_BLUE,
                        &ctx,
                        &msg,
                        None,
                    )
                    .await;
                }
                Err(err) => {
                    // Print an error message if fetching user details fails
                    println!("Error getting user details: {:?}", err);
                }
            }
        } else {
            // Print an error message if the user ID is invalid
            println!("Invalid user ID");
        }
    }

    async fn handle_help(&self, ctx: &Context, msg: &Message) {
        // Send the help message as an embed
        send_embed_message("!help", HELP_MESSAGE, Colour::DARK_GREEN, &ctx, &msg, None).await;
    }

    async fn handle_info(&self, ctx: &Context, msg: &Message) {
        // Send the info message as an embed
        send_embed_message("!info", INFO_MESSAGE, Colour::RED, &ctx, &msg, None).await;
    }

    async fn handle_get_balance(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        let user_id: i32 = 1;
        // Fetch user balance and handle the result
        match get_user_balance(&self.http_client, &self.base_url, user_id).await {
            Ok(balance) => {
                // Format the balance details into a message
                let balance_messsage_details =
                    format!("Your StackUp balance is: ${}", balance.current_balance);
                // Send the formatted message as an embed
                send_embed_message(
                    "!get-balance",
                    &balance_messsage_details,
                    Colour::DARK_PURPLE,
                    &ctx,
                    &msg,
                    None,
                )
                .await;
            }
            Err(err) => {
                // Print an error message if fetching user balance fails
                println!("Error getting user balance: {:?}", err);
            }
        }
    }

    async fn handle_get_profile(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        let user_id: i32 = 1;
        // Fetch user profile and handle the result
        match get_user_progress(&self.http_client, &self.base_url, user_id).await {
            Ok(profile) => {
                // Format the profile details into a message
                let profile_message_details = format!(
                    "Submissions: {}\nSubmitted: {}\nRewarded: {}\nTotal Earnings: ${}",
                    profile.submissions,
                    profile.submitted,
                    profile.rewarded,
                    profile.total_quest_earings
                );
                // Send the formatted message as an embed
                send_embed_message(
                    "!get-profile",
                    &profile_message_details,
                    Colour::DARK_GOLD,
                    &ctx,
                    &msg,
                    None,
                )
                .await;
            }
            Err(err) => {
                // Print an error message if fetching user profile fails
                println!("Error getting user profile: {:?}", err);
            }
        }
    }

    async fn handle_get_campaigns(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        // Fetch featured campaigns and handle the result
        match get_featured_campaigns(&self.http_client, &self.base_url).await {
            Ok(campaigns) => {
                // Initialize the campaign message details
                let mut campaign_message_details = String::from("**Featured campaigns:** \n\n");
                // Append each campaign's details to the message
                for campaign in campaigns {
                    campaign_message_details.push_str(&format!(
                        "**Title:** {}\n**Subtitle:** {}\n**Quest Count:** {}\n\n",
                        campaign.title, campaign.sub_title, campaign.quest_count
                    ));
                }
                // Send the formatted message as an embed
                send_embed_message(
                    "!get-campaigns",
                    &campaign_message_details,
                    Colour::DARK_ORANGE,
                    &ctx,
                    &msg,
                    None,
                )
                .await;
            }
            Err(err) => {
                // Print an error message if fetching campaigns fails
                println!("Error getting featured campaigns: {:?}", err);
            }
        }
    }

    async fn handle_get_pathways(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        // Fetch featured pathways and handle the result
        match get_pathways(&self.http_client, &self.base_url).await {
            Ok(pathways) => {
                // Initialize the pathways message details
                let mut pathways_message_details = String::from("**Featured pathways:** \n\n");
                // Append each pathway's details to the message
                for pathway in pathways {
                    pathways_message_details.push_str(&format!(
                        "**Title:** {}\n**Modules:** {}\n**Skills:** {}\n\n",
                        pathway.title, pathway.modules, pathway.skills
                    ));
                }
                // Send the formatted message as an embed
                send_embed_message(
                    "!get-pathways",
                    &pathways_message_details,
                    Colour::DARK_TEAL,
                    &ctx,
                    &msg,
                    None,
                )
                .await;
            }
            Err(err) => {
                // Print an error message if fetching pathways fails
                println!("Error getting featured pathways: {:?}", err);
            }
        }
    }

    async fn handle_get_hackathons(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        // Fetch upcoming hackathons and handle the result
        match get_hackathons(&self.http_client, &self.base_url).await {
            Ok(hackathons) => {
                // Initialize the hackathons message details
                let mut hackathons_message_details = String::from("**Upcoming hackathons:** \n\n");
                // Append each hackathon's details to the message
                for hackathon in hackathons {
                    hackathons_message_details.push_str(&format!(
                        "**Title:** {}\n**Price:** ${}\n**Participating:** {}\n**Location:** {}\n\n",
                        hackathon.title, hackathon.price, hackathon.participating, hackathon.location
                    ));
                }
                // Send the formatted message as an embed
                send_embed_message(
                    "!get-hackathons",
                    &hackathons_message_details,
                    Colour::DARK_RED,
                    &ctx,
                    &msg,
                    None,
                )
                .await;
            }
            Err(err) => {
                // Print an error message if fetching hackathons fails
                println!("Error getting upcoming hackathons: {:?}", err);
            }
        }
    }

    async fn handle_get_calendar(&self, ctx: &Context, msg: &Message) {
        // Show loading animation
        let loading_msg = show_loading_animation(ctx, msg, "Loading...").await;

        // Define the calendar link and image URL
        let calendar_link = "https://stackup.dev/calendar";
        const CALENDAR_IMG_URL: &str = "https://i.imgur.com/hxxfDQ9.png";
        // Define the calendar message
        const CALENDAR_MESSAGE: &str = "
            Check out the latest activities happening on the **StackUp platform**, the platform for developers, where you can learn, earn and create projects.
        ";
        // Format the calendar details into a message
        let calendar_details =
            format!("{} [Monthly Calendar]({})", CALENDAR_MESSAGE, calendar_link);
        // Send the formatted message as an embed with an image
        send_embed_message(
            "!get-calendar",
            &calendar_details,
            Colour::DARK_BLUE,
            &ctx,
            &msg,
            Some(CALENDAR_IMG_URL),
        )
        .await;
    }
}
