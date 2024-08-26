use crate::models::*;
use reqwest::Client as ReqwestClient;
use std::error::Error;

// Function to get user balance from the API
pub async fn get_user_balance(
    client: &ReqwestClient,
    base_url: &str,
    user_id: i32,
) -> Result<UserBalance, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/get-user-balance/{}", base_url, user_id);
    let response = client.get(&url).send().await?.json::<UserBalance>().await?;

    Ok(response)
}

// Function to get user profile from the API
pub async fn get_user_progress(
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

// Function to get user details from the API
pub async fn get_user_details(
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

// Function to get featured campaigns from the API
pub async fn get_featured_campaigns(
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

// Function to get pathways from the API
pub async fn get_pathways(
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

// Function to get hackathons from the API
pub async fn get_hackathons(
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
