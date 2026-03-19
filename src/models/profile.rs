use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserProfile {
    pub user_profile: ProfileDetails,
}

#[derive(Debug, Deserialize)]
pub struct ProfileDetails {
    pub username: String,
    pub display_name: String,
    pub age: u8,
    pub gender: String,
    pub location: String,
    pub education: Education,
    pub occupation: String,
    pub mbti: String,
    pub interests: Vec<String>,
    pub looking_for: LookingFor,
}

#[derive(Debug, Deserialize)]
pub struct Education {
    pub degree: String,
    pub university: String,
}

#[derive(Debug, Deserialize)]
pub struct LookingFor {
    pub gender_preference: String,
    pub age_range: AgeRange,
    pub relationship_type: String,
    pub key_traits: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AgeRange {
    pub min: u8,
    pub max: u8,
}

pub struct ParseResult {
    pub profile: UserProfile,
    pub content: String,
}
