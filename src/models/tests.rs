#[cfg(test)]
mod tests {
    use crate::models::profile::{AgeRange, Education, LookingFor, ProfileDetails, UserProfile};

    #[test]
    fn test_age_range_creation() {
        let age_range = AgeRange { min: 20, max: 30 };
        assert_eq!(age_range.min, 20);
        assert_eq!(age_range.max, 30);
    }

    #[test]
    fn test_age_range_validity() {
        let age_range = AgeRange { min: 25, max: 35 };
        assert!(age_range.min < age_range.max);
    }

    #[test]
    fn test_education_creation() {
        let education = Education {
            degree: "Bachelor of Science".to_string(),
            university: "Test University".to_string(),
        };
        assert_eq!(education.degree, "Bachelor of Science");
        assert_eq!(education.university, "Test University");
    }

    #[test]
    fn test_looking_for_creation() {
        let looking_for = LookingFor {
            gender_preference: "Female".to_string(),
            age_range: AgeRange { min: 20, max: 30 },
            relationship_type: "Serious".to_string(),
            key_traits: vec!["Kind".to_string(), "Intelligent".to_string()],
        };

        assert_eq!(looking_for.gender_preference, "Female");
        assert_eq!(looking_for.age_range.min, 20);
        assert_eq!(looking_for.age_range.max, 30);
        assert_eq!(looking_for.relationship_type, "Serious");
        assert_eq!(looking_for.key_traits.len(), 2);
    }

    #[test]
    fn test_profile_details_creation() {
        let profile = ProfileDetails {
            username: "test_user".to_string(),
            display_name: "Test User".to_string(),
            age: 28,
            gender: "Male".to_string(),
            location: "Bangkok".to_string(),
            education: Education {
                degree: "Bachelor".to_string(),
                university: "University".to_string(),
            },
            occupation: "Engineer".to_string(),
            mbti: "INTP".to_string(),
            interests: vec!["Coding".to_string(), "Gaming".to_string()],
            looking_for: LookingFor {
                gender_preference: "Female".to_string(),
                age_range: AgeRange { min: 24, max: 32 },
                relationship_type: "Serious".to_string(),
                key_traits: vec!["Intelligent".to_string()],
            },
        };

        assert_eq!(profile.username, "test_user");
        assert_eq!(profile.display_name, "Test User");
        assert_eq!(profile.age, 28);
        assert_eq!(profile.gender, "Male");
        assert_eq!(profile.location, "Bangkok");
        assert_eq!(profile.occupation, "Engineer");
        assert_eq!(profile.mbti, "INTP");
        assert_eq!(profile.interests.len(), 2);
    }

    #[test]
    fn test_user_profile_creation() {
        let user_profile = UserProfile {
            user_profile: ProfileDetails {
                username: "alice".to_string(),
                display_name: "Alice Smith".to_string(),
                age: 26,
                gender: "Female".to_string(),
                location: "Singapore".to_string(),
                education: Education {
                    degree: "Master".to_string(),
                    university: "NUS".to_string(),
                },
                occupation: "Data Scientist".to_string(),
                mbti: "ENFP".to_string(),
                interests: vec![],
                looking_for: LookingFor {
                    gender_preference: "Male".to_string(),
                    age_range: AgeRange { min: 26, max: 40 },
                    relationship_type: "Long-term".to_string(),
                    key_traits: vec![],
                },
            },
        };

        assert_eq!(user_profile.user_profile.username, "alice");
        assert_eq!(user_profile.user_profile.display_name, "Alice Smith");
        assert_eq!(user_profile.user_profile.age, 26);
    }

    #[test]
    fn test_empty_interests_list() {
        let profile = ProfileDetails {
            username: "no_interests".to_string(),
            display_name: "No Interests User".to_string(),
            age: 30,
            gender: "Male".to_string(),
            location: "Test City".to_string(),
            education: Education {
                degree: "Degree".to_string(),
                university: "University".to_string(),
            },
            occupation: "Job".to_string(),
            mbti: "MBTI".to_string(),
            interests: vec![],
            looking_for: LookingFor {
                gender_preference: "Female".to_string(),
                age_range: AgeRange { min: 25, max: 35 },
                relationship_type: "Casual".to_string(),
                key_traits: vec![],
            },
        };

        assert_eq!(profile.interests.len(), 0);
    }

    #[test]
    fn test_profile_with_many_interests() {
        let interests = vec![
            "Reading".to_string(),
            "Hiking".to_string(),
            "Cooking".to_string(),
            "Music".to_string(),
            "Art".to_string(),
        ];

        let profile = ProfileDetails {
            username: "many_interests".to_string(),
            display_name: "Many Interests".to_string(),
            age: 28,
            gender: "Female".to_string(),
            location: "Test City".to_string(),
            education: Education {
                degree: "Degree".to_string(),
                university: "University".to_string(),
            },
            occupation: "Job".to_string(),
            mbti: "MBTI".to_string(),
            interests: interests.clone(),
            looking_for: LookingFor {
                gender_preference: "Male".to_string(),
                age_range: AgeRange { min: 25, max: 35 },
                relationship_type: "Serious".to_string(),
                key_traits: vec![],
            },
        };

        assert_eq!(profile.interests.len(), 5);
        assert!(profile.interests.contains(&"Reading".to_string()));
        assert!(profile.interests.contains(&"Music".to_string()));
    }
}
