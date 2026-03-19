use gitder::parser::yaml::parse_profile_file;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_parse_complete_profile_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let content = r#"---
user_profile:
  username: integration_test_user
  display_name: Integration Test User
  age: 30
  gender: Male
  location: Integration Test City
  education:
    degree: Bachelor of Software Engineering
    university: Test Technology University
  occupation: Senior Developer
  mbti: INFP
  interests:
    - Software Development
    - Machine Learning
    - Web Development
  looking_for:
    gender_preference: Female
    age_range:
      min: 25
      max: 35
    relationship_type: Long-term
    key_traits:
      - Supportive
      - Intelligent
      - Creative
---
# About Me

I am passionate about technology and innovation.

## Skills
- Rust
- Python
- JavaScript
"#;

    let file_path = temp_dir.path().join("integration_test.md");
    fs::write(&file_path, content).expect("Failed to write test file");

    // Test parsing
    let result = parse_profile_file(&file_path);
    assert!(result.is_ok(), "Failed to parse integration test profile");

    let parse_result = result.unwrap();

    // Verify profile data
    assert_eq!(
        parse_result.profile.user_profile.username,
        "integration_test_user"
    );
    assert_eq!(
        parse_result.profile.user_profile.display_name,
        "Integration Test User"
    );
    assert_eq!(parse_result.profile.user_profile.age, 30);
    assert_eq!(parse_result.profile.user_profile.gender, "Male");
    assert_eq!(
        parse_result.profile.user_profile.location,
        "Integration Test City"
    );

    // Verify education
    assert_eq!(
        parse_result.profile.user_profile.education.degree,
        "Bachelor of Software Engineering"
    );
    assert_eq!(
        parse_result.profile.user_profile.education.university,
        "Test Technology University"
    );

    // Verify occupation and MBTI
    assert_eq!(
        parse_result.profile.user_profile.occupation,
        "Senior Developer"
    );
    assert_eq!(parse_result.profile.user_profile.mbti, "INFP");

    // Verify interests
    assert_eq!(parse_result.profile.user_profile.interests.len(), 3);
    assert!(
        parse_result
            .profile
            .user_profile
            .interests
            .contains(&"Software Development".to_string())
    );

    // Verify looking for
    let looking_for = &parse_result.profile.user_profile.looking_for;
    assert_eq!(looking_for.gender_preference, "Female");
    assert_eq!(looking_for.age_range.min, 25);
    assert_eq!(looking_for.age_range.max, 35);
    assert_eq!(looking_for.relationship_type, "Long-term");
    assert_eq!(looking_for.key_traits.len(), 3);

    // Verify content extraction
    assert!(parse_result.content.contains("# About Me"));
    assert!(parse_result.content.contains("Rust"));
}

#[test]
fn test_multiple_profiles_parsing() {
    let temp_dir = TempDir::new().unwrap();

    // Create first profile
    let profile1_content = r#"---
user_profile:
  username: user1
  display_name: User One
  age: 25
  gender: Male
  location: City 1
  education:
    degree: Degree 1
    university: University 1
  occupation: Job 1
  mbti: INTP
  interests: []
  looking_for:
    gender_preference: Female
    age_range:
      min: 20
      max: 30
    relationship_type: Serious
    key_traits: []
---
"#;

    // Create second profile
    let profile2_content = r#"---
user_profile:
  username: user2
  display_name: User Two
  age: 28
  gender: Female
  location: City 2
  education:
    degree: Degree 2
    university: University 2
  occupation: Job 2
  mbti: ENFP
  interests: []
  looking_for:
    gender_preference: Male
    age_range:
      min: 25
      max: 35
    relationship_type: Long-term
    key_traits: []
---
"#;

    let file1 = temp_dir.path().join("profile1.md");
    let file2 = temp_dir.path().join("profile2.md");

    fs::write(&file1, profile1_content).expect("Failed to write profile 1");
    fs::write(&file2, profile2_content).expect("Failed to write profile 2");

    // Parse both files
    let result1 = parse_profile_file(&file1).expect("Failed to parse profile 1");
    let result2 = parse_profile_file(&file2).expect("Failed to parse profile 2");

    // Verify they are different
    assert_eq!(result1.profile.user_profile.username, "user1");
    assert_eq!(result2.profile.user_profile.username, "user2");
    assert_ne!(
        result1.profile.user_profile.display_name,
        result2.profile.user_profile.display_name
    );
}
