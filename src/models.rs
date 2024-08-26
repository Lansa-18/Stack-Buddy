use serde::Deserialize;

// Struct Format of API Response for getting user details
#[derive(Deserialize, Debug)]
pub struct GetUserResponse {
    pub id: i32,
    pub username: String,
    pub nationality: String,
    pub career_level: String,
    pub role: String,
    pub tech_stack: String,
}

// Struct Format of API Response for getting user balance
#[derive(Deserialize)]
pub struct UserBalance {
    id: i32,
    user_id: String,
    total_earnings: i32,
    total_withdrawn: i32,
    withdrawal_methods: String,
    pub current_balance: i32,
    created_at: String,
    updated_at: String,
}

// Struct Format of API Response for getting user profile
#[derive(Deserialize)]
pub struct UserProgress {
    id: i32,
    user_id: String,
    pub submissions: i32,
    pub submitted: i32,
    pub rewarded: i32,
    pub total_quest_earings: i32,
}

// Struct Format of API Response for getting featured campaigns
#[derive(Deserialize)]
pub struct Campaigns {
    id: i32,
    pub title: String,
    pub sub_title: String,
    pub quest_count: i32,
}

// Struct Format of API Response for getting pathways
#[derive(Deserialize)]
pub struct Pathways {
    id: i32,
    pub title: String,
    pub modules: i32,
    pub skills: i32,
}

// Struct Format of API Response for getting hackathons
#[derive(Deserialize)]
pub struct Hackathons {
    id: i32,
    pub title: String,
    pub price: i32,
    pub participating: i32,
    pub location: i32, 
}
