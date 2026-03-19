#[cfg(test)]
mod tests {
    use crate::parser::yaml::parse_profile_file;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_profile(dir: &TempDir, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(filename);
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path
    }

    #[test]
    fn test_parse_valid_profile() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: john_doe
  display_name: John Doe
  age: 28
  gender: Male
  location: Bangkok, Thailand
  education:
    degree: Bachelor of Computer Science
    university: Chulalongkorn University
  occupation: Software Engineer
  mbti: INTP
  interests:
    - Programming
    - Gaming
    - Travel
  looking_for:
    gender_preference: Female
    age_range:
      min: 24
      max: 32
    relationship_type: Serious
    key_traits:
      - Intelligent
      - Ambitious
---
This is the main content.
"#;

        let file_path = create_test_profile(&temp_dir, "test.md", content);
        let result = parse_profile_file(&file_path);

        assert!(result.is_ok(), "Failed to parse valid profile");
        let parse_result = result.unwrap();
        assert_eq!(parse_result.profile.user_profile.username, "john_doe");
        assert_eq!(parse_result.profile.user_profile.display_name, "John Doe");
        assert_eq!(parse_result.profile.user_profile.age, 28);
        assert_eq!(parse_result.profile.user_profile.gender, "Male");
        assert_eq!(
            parse_result.profile.user_profile.location,
            "Bangkok, Thailand"
        );
        assert_eq!(
            parse_result.profile.user_profile.occupation,
            "Software Engineer"
        );
        assert_eq!(parse_result.profile.user_profile.mbti, "INTP");
        assert_eq!(parse_result.profile.user_profile.interests.len(), 3);
        assert!(parse_result.content.contains("This is the main content"));
    }

    #[test]
    fn test_parse_education() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: test_user
  display_name: Test User
  age: 25
  gender: Female
  location: Bangkok
  education:
    degree: Master of Data Science
    university: Kasetsart University
  occupation: Data Scientist
  mbti: INTJ
  interests: []
  looking_for:
    gender_preference: Male
    age_range:
      min: 25
      max: 35
    relationship_type: Casual
    key_traits: []
---
"#;

        let file_path = create_test_profile(&temp_dir, "test.md", content);
        let result = parse_profile_file(&file_path).unwrap();

        assert_eq!(
            result.profile.user_profile.education.degree,
            "Master of Data Science"
        );
        assert_eq!(
            result.profile.user_profile.education.university,
            "Kasetsart University"
        );
    }

    #[test]
    fn test_parse_looking_for() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: jane_doe
  display_name: Jane Doe
  age: 26
  gender: Female
  location: Singapore
  education:
    degree: Bachelor
    university: National University of Singapore
  occupation: Engineer
  mbti: ENFP
  interests: []
  looking_for:
    gender_preference: Male
    age_range:
      min: 26
      max: 40
    relationship_type: Long-term
    key_traits:
      - Honest
      - Caring
      - Motivated
---
"#;

        let file_path = create_test_profile(&temp_dir, "test.md", content);
        let result = parse_profile_file(&file_path).unwrap();

        let looking_for = &result.profile.user_profile.looking_for;
        assert_eq!(looking_for.gender_preference, "Male");
        assert_eq!(looking_for.age_range.min, 26);
        assert_eq!(looking_for.age_range.max, 40);
        assert_eq!(looking_for.relationship_type, "Long-term");
        assert_eq!(looking_for.key_traits.len(), 3);
        assert!(looking_for.key_traits.contains(&"Honest".to_string()));
    }

    #[test]
    fn test_parse_missing_frontmatter() {
        let temp_dir = TempDir::new().unwrap();
        let content = "This is just plain content without YAML frontmatter";
        let file_path = create_test_profile(&temp_dir, "test.md", content);

        let result = parse_profile_file(&file_path);
        assert!(
            result.is_err(),
            "Should fail when no YAML frontmatter found"
        );
    }

    #[test]
    fn test_parse_nonexistent_file() {
        let result = parse_profile_file("/nonexistent/path/to/file.md");
        assert!(result.is_err(), "Should fail when file does not exist");
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: john
  invalid_yaml_here: [this is broken yaml
---
"#;
        let file_path = create_test_profile(&temp_dir, "test.md", content);

        let result = parse_profile_file(&file_path);
        assert!(result.is_err(), "Should fail with invalid YAML");
    }

    #[test]
    fn test_parse_content_extraction() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: test
  display_name: Test
  age: 30
  gender: Male
  location: Test City
  education:
    degree: Test Degree
    university: Test University
  occupation: Tester
  mbti: TEST
  interests: []
  looking_for:
    gender_preference: Female
    age_range:
      min: 25
      max: 35
    relationship_type: Test
    key_traits: []
---
# This is my bio
I love testing things!
More content here.
"#;

        let file_path = create_test_profile(&temp_dir, "test.md", content);
        let result = parse_profile_file(&file_path).unwrap();

        assert!(result.content.contains("# This is my bio"));
        assert!(result.content.contains("I love testing things!"));
        assert!(result.content.contains("More content here."));
    }

    #[test]
    fn test_parse_empty_interests() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: no_interests
  display_name: No Interests
  age: 30
  gender: Male
  location: Test City
  education:
    degree: Degree
    university: University
  occupation: Job
  mbti: MBTI
  interests: []
  looking_for:
    gender_preference: Female
    age_range:
      min: 25
      max: 35
    relationship_type: Casual
    key_traits: []
---
"#;

        let file_path = create_test_profile(&temp_dir, "test.md", content);
        let result = parse_profile_file(&file_path).unwrap();

        assert_eq!(result.profile.user_profile.interests.len(), 0);
    }

    #[test]
    fn test_parse_multiple_interests() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
user_profile:
  username: many_interests
  display_name: Many Interests
  age: 25
  gender: Female
  location: Test City
  education:
    degree: Degree
    university: University
  occupation: Job
  mbti: MBTI
  interests:
    - Reading
    - Hiking
    - Cooking
    - Music
    - Art
    - Sports
  looking_for:
    gender_preference: Male
    age_range:
      min: 25
      max: 35
    relationship_type: Long-term
    key_traits: []
---
"#;

        let file_path = create_test_profile(&temp_dir, "test.md", content);
        let result = parse_profile_file(&file_path).unwrap();

        assert_eq!(result.profile.user_profile.interests.len(), 6);
        assert!(
            result
                .profile
                .user_profile
                .interests
                .contains(&"Reading".to_string())
        );
        assert!(
            result
                .profile
                .user_profile
                .interests
                .contains(&"Hiking".to_string())
        );
    }
}
