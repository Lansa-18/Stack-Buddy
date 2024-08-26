use crate::api::{
    get_featured_campaigns, get_hackathons, get_pathways, get_user_balance, get_user_details,
    get_user_progress,
};
use crate::discord::discord::send_embed_message;
use crate::discord::messages::{HELP_MESSAGE, INFO_MESSAGE};
use crate::util::show_loading_animation;
use serenity::model::colour::Colour;
use serenity::model::prelude::*;
use serenity::prelude::*;
