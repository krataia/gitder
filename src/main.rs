use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use gitder::parser::yaml::parse_profile_file;
use std::{fs, io};
use termimad::MadSkin;

fn main() -> io::Result<()> {
    let base_path = "./lib/domains";

    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let country_code = path.file_name().unwrap().to_string_lossy().to_uppercase();

            println!("\n🌍 [ COUNTRY: {} ]", country_code);

            for sub_entry in fs::read_dir(&path)? {
                let sub_entry = sub_entry?;
                let sub_path = sub_entry.path();

                if sub_path.is_dir() {
                    let readme_path = sub_path.join("README.md");
                    if readme_path.exists()
                        && let Ok(result) = parse_profile_file(&readme_path) {
                            render_profile_table(result);
                        }
                }
            }
        }
    }
    Ok(())
}

fn render_profile_table(res: gitder::models::profile::ParseResult) {
    let p = res.profile.user_profile;
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80);
    table.add_row(vec![
        Cell::new("👤 PROFILE CARD")
            .add_attribute(Attribute::Bold)
            .fg(Color::Yellow),
        Cell::new(format!("{} (@{})", p.display_name, p.username))
            .add_attribute(Attribute::Bold)
            .fg(Color::Green),
    ]);

    table.add_row(vec!["📍 Location", &p.location]);
    table.add_row(vec![
        "🎂 Age / Gender",
        &format!("{} / {}", p.age, p.gender),
    ]);
    table.add_row(vec![
        "🧠 MBTI / Job",
        &format!("{} / {}", p.mbti, p.occupation),
    ]);
    table.add_row(vec![
        "🎓 Education",
        &format!("{} ({})", p.education.degree, p.education.university),
    ]);
    table.add_row(vec!["🎨 Interests", &p.interests.join(", ")]);

    let looking_for = format!(
        "Type: {}\nAge: {} - {}\nTraits: {}",
        p.looking_for.relationship_type,
        p.looking_for.age_range.min,
        p.looking_for.age_range.max,
        p.looking_for.key_traits.join(", ")
    );
    table.add_row(vec!["🎯 Looking For", &looking_for]);
    println!("{table}");
    let skin = MadSkin::default();
    skin.print_text(&format!("*{}*", res.content.trim()));
    let mut separator = Table::new();
    separator
        .load_preset(comfy_table::presets::NOTHING)
        .set_width(80)
        .add_row(vec![Cell::new("═".repeat(80)).fg(Color::DarkGrey)]);
    println!("{separator}");
}
