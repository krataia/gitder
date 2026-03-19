use gray_matter::Matter;
use gray_matter::engine::YAML;

use crate::models::profile::{ParseResult, UserProfile};
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn parse_profile_file<P: AsRef<Path>>(path: P) -> Result<ParseResult, Box<dyn Error>> {
    let raw_content = fs::read_to_string(&path)
        .map_err(|e| format!("read file failed {}: {}", path.as_ref().display(), e))?;

    let matter = Matter::<YAML>::new();

    let parsed = matter
        .parse::<UserProfile>(&raw_content)
        .map_err(|e| format!("parse failed: {}", e))?;

    let profile = parsed
        .data
        .ok_or("No YAML Frontmatter found in file".to_string())?;

    Ok(ParseResult {
        profile,
        content: parsed.content,
    })
}
