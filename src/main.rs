use dotenv::dotenv;
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use serenity::Client;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage},
    model::colour::Colour,
    model::Timestamp,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::error::Error;
use tokio;

// Messages
const HELP_MESSAGE: &str = "
**🌟 GM Stackies! 🌟**

I'm your **go-to stack-buddy**! Want to get information from the StackUp platform? Here are my commands:

────────────────────────

**📋 Commands:**

• **`!help`** - *Get a list of all commands.*
• **`!info`** - *Learn more about what I can do.*
• **`!get-username`** - *Retrieve your StackUp username.*
• **`!get-balance`** - *Check your StackUp balance.*
• **`!get-profile`** - *Check your StackUp profile.*
• **`!get-campaigns`** - *View Featured campaigns.* 
• **`!get-pathways`** - *View various resources for various tracks.* 
• **`!get-hackathons`** - *See upcoming hackathons.*
• **`!get-calendar`** - *Access the StackUp calendar.*

────────────────────────

Let's get stacking! 📚

— **Stack-Buddy** 🤖
";

const INFO_MESSAGE: &str = "
**Hello!** I'm **Stack-Buddy**, your personal assistant for all things **StackUp**.  
I serve as an intermediary between you and the StackUp platform, making it easy to get the information 
you need directly from the StackUp website.

With a few simple commands, I can fetch and display the latest updates,details, and resources from StackUp
right here for you.

**Features:**

📜 **Information Fetching:** Get the latest details and updates from StackUp.  
🔍 **Resource Access:** Quickly access important resources and documents.  
🛠️ **User Assistance:** Providing support and guidance for StackUp-related queries.

*Just type a command, and I'll do the rest!*

— **Stack-Buddy** 🤖
";

// Struct Format of API Response for getting user details
#[derive(Deserialize, Debug)]
struct GetUserResponse {
    id: i32,
    username: String,
    nationality: String,
    career_level: String,
    role: String,
    tech_stack: String,
}

// Function to get user details from the API
async fn get_user_details(
    client: &ReqwestClient,
    base_url: &str,
    user_id: i32,
) -> Result<GetUserResponse, reqwest::Error> {
    let url = format!("{}/get-user/{}", base_url, user_id);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<GetUserResponse>()
        .await?;

    Ok(response)
}

// Struct Format of API Response for getting user balance
#[derive(Deserialize)]
struct UserBalance {
    id: i32,
    user_id: String,
    total_earnings: i32,
    total_withdrawn: i32,
    current_balance: i32,
    withdrawal_methods: String,
    created_at: String,
    updated_at: String,
}

// Function to get user balance from the API
async fn get_user_balance(
    client: &ReqwestClient,
    base_url: &str,
    user_id: i32,
) -> Result<UserBalance, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/get-user-balance/{}", base_url, user_id);
    let response = client.get(&url).send().await?.json::<UserBalance>().await?;

    Ok(response)
}

// Struct Format of API Response for getting user profile
#[derive(Deserialize)]
struct UserProgress {
    id: i32,
    user_id: String,
    submissions: i32,
    submitted: i32,
    rewarded: i32,
    total_quest_earings: i32,
}

// Function to get user profile from the API
async fn get_user_progress(
    client: &ReqwestClient,
    base_url: &str,
    user_id: i32,
) -> Result<UserProgress, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/get-user-progress/{}", base_url, user_id);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<UserProgress>()
        .await?;

    Ok(response)
}

// Struct Format of API Response for getting featured campaigns
#[derive(Deserialize)]
struct Campaigns {
    // Later discuss with mr moses to add a reward_pool field to the campaign endpoint
    id: i32,
    title: String,
    sub_title: String,
    quest_count: i32,
}

// Function to get featured campaigns from the API
async fn get_featured_campaigns(
    client: &ReqwestClient,
    base_url: &str,
) -> Result<Vec<Campaigns>, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/stack-featured-campaigns", base_url);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<Campaigns>>()
        .await?;

    Ok(response)
}

// Struct Format of API Response for getting pathways
#[derive(Deserialize)]
struct Pathways {
    id: i32,
    title: String,
    modules: i32,
    skills: i32,
}

// Function to get pathways from the API
async fn get_pathways(
    client: &ReqwestClient,
    base_url: &str,
) -> Result<Vec<Pathways>, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/stack-featured-pathways", base_url);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<Pathways>>()
        .await?;

    Ok(response)
}

// Struct Format of API Response for getting hackathons
#[derive(Deserialize)]
struct Hackathons {
    id: i32,
    title: String,
    price: i32,
    participating: i32,
    location: i32, // tell mr moses to change the location to virtual/online or physical
}

// Function to get hackathons from the API
async fn get_hackathons(
    client: &ReqwestClient,
    base_url: &str,
) -> Result<Vec<Hackathons>, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/stack-featured-hackathons", base_url);
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<Vec<Hackathons>>()
        .await?;

    Ok(response)
}

// Creating a function to display embedded messages
async fn send_embed_message(
    command_name: &str,
    message_details: &str,
    color: Colour,
    ctx: &Context,
    msg: &Message,
    image_url: Option<&str>,
) {
    if msg.content == command_name {
        let author = CreateEmbedAuthor::new("Stack-Buddy");

        let mut embed = CreateEmbed::default()
            .author(author)
            .description(message_details)
            .thumbnail("https://i.imgur.com/xRT3jsZ.png")
            .timestamp(Timestamp::now())
            .color(color);

        if let Some(url) = image_url {
            embed = embed.image(url);
        }

        let message = CreateMessage::default().embed(embed);

        if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

struct Handler {
    http_client: ReqwestClient,
    base_url: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.trim();
        match content {
            "!get-username" => {
                let user_id: i32 = 1;
                if user_id > 0 {
                    match get_user_details(&self.http_client, &self.base_url, user_id).await {
                        Ok(user) => {
                            let message_details = format!(
                                "ID: {}\nUsername: {}\nNationality: {}\nCareer Level: {}\nRole: {}\nTech Stacks: {}",
                                user.id, user.username, user.nationality, user.career_level, user.role, user.tech_stack
                            );
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
                            println!("Error getting user details: {:?}", err);
                        }
                    }
                } else {
                    println!("Invalid user ID");
                }
            }
            "!help" => {
                send_embed_message("!help", HELP_MESSAGE, Colour::DARK_GREEN, &ctx, &msg, None)
                    .await;
            }
            "!info" => {
                send_embed_message("!info", INFO_MESSAGE, Colour::RED, &ctx, &msg, None).await;
            }
            "!get-balance" => {
                let user_id: i32 = 1;
                match get_user_balance(&self.http_client, &self.base_url, user_id).await {
                    Ok(balance) => {
                        let balance_messsage_details =
                            format!("Your StackUp balance is: ${}", balance.current_balance);
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
                        println!("Error getting user balance: {:?}", err);
                    }
                }
            }
            "!get-profile" => {
                let user_id: i32 = 1;
                match get_user_progress(&self.http_client, &self.base_url, user_id).await {
                    Ok(profile) => {
                        let profile_message_details = format!(
                            "Submissions: {}\nSubmitted: {}\nRewarded: {}\nTotal Earnings: ${}",
                            profile.submissions,
                            profile.submitted,
                            profile.rewarded,
                            profile.total_quest_earings
                        );
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
                        println!("Error getting user profile: {:?}", err);
                    }
                }
            }
            "!get-campaigns" => {
                match get_featured_campaigns(&self.http_client, &self.base_url).await {
                    Ok(campaigns) => {
                        let mut campaign_message_details =
                            String::from("**Featured campaigns:** \n\n");
                        for campaign in campaigns {
                            campaign_message_details.push_str(&format!(
                                "**Title:** {}\n**Subtitle:** {}\n**Quest Count:** {}\n\n",
                                campaign.title, campaign.sub_title, campaign.quest_count
                            ));
                        }
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
                        println!("Error getting featured campaigns: {:?}", err);
                    }
                }
            }
            "!get-pathways" => match get_pathways(&self.http_client, &self.base_url).await {
                Ok(pathways) => {
                    let mut pathways_message_details = String::from("**Featured pathways:** \n\n");
                    for pathway in pathways {
                        pathways_message_details.push_str(&format!(
                            "**Title:** {}\n**Modules:** {}\n**Skills:** {}\n\n",
                            pathway.title, pathway.modules, pathway.skills
                        ));
                    }
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
                    println!("Error getting featured pathways: {:?}", err);
                }
            },
            "!get-hackathons" => match get_hackathons(&self.http_client, &self.base_url).await {
                Ok(hackathons) => {
                    let mut hackathons_message_details =
                        String::from("**Upcoming hackathons:** \n\n");
                    for hackathon in hackathons {
                        hackathons_message_details.push_str(&format!(
                                "**Title:** {}\n**Price:** ${}\n**Participating:** {}\n**Location:** {}\n\n",
                                hackathon.title, hackathon.price, hackathon.participating, hackathon.location
                            ));
                    }
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
                    println!("Error getting upcoming hackathons: {:?}", err);
                }
            },
            "!get-calendar" => {
                let calendar_link = "https://stackup.dev/calendar";
                const CALENDAR_IMG_URL: &str = "https://i.imgur.com/hxxfDQ9.png";
                const CALENDAR_MESSAGE: &str = "
                    Check out the latest activities happening on the **StackUp platform**, the platform for developers, where you can learn, earn and create projects.
                ";
                let calendar_details = format!(
                    "{} [Monthly Calendar]({})",
                    CALENDAR_MESSAGE, calendar_link
                );
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
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let base_url = "https://superna.ytechno.com.ng/api";
    let http_client = ReqwestClient::new();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            http_client,
            base_url: base_url.to_string(),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
