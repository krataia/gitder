# Gitder 💔🔥 - GitHub Tinder Parody

A Rust CLI application that brings the swiping experience to GitHub developers. **Gitder** is a humorous parody of Tinder designed specifically to match developers based on their GitHub profiles, tech stacks, and collaboration preferences.

## 📋 Overview

Gitder is a profile parser and display system that reads YAML front matter from markdown files and renders them as beautifully formatted tables in the terminal. Think of it as finding your perfect coding partner (or worst nightmare) - swipe through developer profiles organized by country and region, check their tech preferences, and see if you're a match! 

No actual swiping involved (yet), but plenty of sass and developer culture. 🚀

## 🏗️ Project Structure

```
gitder/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library exports
│   ├── models/
│   │   ├── mod.rs
│   │   └── profile.rs          # User profile data structures
│   └── parser/
│       ├── mod.rs
│       └── yaml.rs             # YAML parsing logic
├── lib/
│   └── domains/                # Profile files organized by country code
│       └── th/
│           └── your_name/
│               └── README.md   # Profile with YAML front matter
├── Cargo.toml
└── README.md
```

## 📦 Data Models

### UserProfile
A GitHub developer's profile structure containing:

- **GitHub Info**: username, display_name (real name), GitHub avatar location
- **Developer Details**: age, gender, location (for community meetups!)
- **Education**: degree, university (where they learned to code)
- **Career**: occupation, tech stack, favorite programming languages
- **Personality**: MBTI type (because developers love personality typing!)
- **Interests**: Open source projects, tech communities, side hustles
- **Collaboration Preferences**: Looking for - partner tech stack, experience level, project types, values in a dev partner

### Profile Structure Example

```yaml
---
user_profile:
  username: awesome_dev
  display_name: Alex Chen
  age: 28
  gender: Non-binary
  location: Bangkok, Thailand
  education:
    degree: Bachelor of Computer Science
    university: Chulalongkorn University
  occupation: Full-stack Developer
  mbti: INTP
  interests:
    - Rust & Systems Programming
    - Open Source Contribution
    - Machine Learning
    - DevOps & Cloud Architecture
  looking_for:
    gender_preference: Everyone (we're inclusive!)
    age_range:
      min: 18
      max: 65
    relationship_type: "Looking for: Open source collaborators"
    key_traits:
      - Strong fundamentals
      - Loves learning
      - Good at code reviews
      - Humble and patient
---

## About Me

I'm a passionate Rust developer who believes in clean code and good documentation. 
Love contributing to open source and mentoring junior developers!

### My Tech Stack
- **Languages**: Rust, Python, JavaScript, Go
- **Frameworks**: Actix, FastAPI, React, Next.js
- **Databases**: PostgreSQL, Redis, MongoDB
- **DevOps**: Docker, Kubernetes, GitHub Actions

### Projects
- [awesome-project](https://github.com/awesome_dev/awesome-project) - An amazing Rust CLI tool
- [learning-ai](https://github.com/awesome_dev/learning-ai) - ML experiments with PyTorch

### Let's Connect
Looking for collab on open source projects, especially ones involving systems programming or ML!
```

## 🔧 Technologies

- **Language**: Rust 2024 Edition
- **Key Dependencies**:
  - `serde` & `serde_yaml`: Data serialization/deserialization
  - `comfy_table`: Terminal table rendering
  - `gray_matter`: YAML front matter parsing
  - `colored`: Terminal color output
  - `termimad`: Markdown terminal rendering

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (uses 2024 edition)
- Cargo
- A sense of humor about GitHub and the developer community

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
```

The application will scan the `./lib/domains` directory and display all developer profiles in formatted tables, organized by country code. Start swiping (mentally)! 😄

## 📂 Directory Organization

Developer profiles should be organized as follows:

```
lib/domains/
└── {COUNTRY_CODE}/           # Two-letter country code (e.g., 'th' for Thailand, 'us' for US)
    └── {github_username}/     # Each developer in their own folder
        └── README.md          # Their profile with YAML front matter
```

**Example**: `lib/domains/th/awesome_dev/README.md`

Each profile is like a mini GitHub portfolio - showcase your skills, interests, and what you're looking to collaborate on!

## 💻 Code Architecture

### Main Flow
1. **Discovery**: Scans `./lib/domains` for country directories
2. **Processing**: For each country, discovers profile directories
3. **Parsing**: Reads and parses `README.md` files with YAML front matter
4. **Rendering**: Displays each profile in a formatted UTF-8 table

### Key Components

**parser/yaml.rs**: 
- `parse_profile_file()`: Reads a markdown file and extracts YAML front matter into a `UserProfile` struct

**models/profile.rs**:
- Defines all data structures with serde-compatible `#[derive]` attributes
- Ensures type safety across the application

**main.rs**:
- Orchestrates file discovery and profile rendering
- Uses `comfy_table` for beautiful terminal output

## 📝 Usage Example

After setting up profiles in the `lib/domains` directory structure, run:

```bash
cargo run
```

Output:
```
🌍 [ COUNTRY: TH ]
┌─────────────────────────────────┐
│ 👤 PROFILE CARD                 │
│ John Doe (@john_doe)            │
│ 28 years old, Male              │
│ Bangkok, Thailand               │
│ Software Engineer               │
│...
```

## 🛠️ Development

Run tests:
```bash
cargo test
```

Check code formatting:
```bash
cargo fmt --check
```

Lint code:
```bash
cargo clippy
```

## 📄 License

This project is open source. Modify and distribute as needed.

## 🤝 Contributing

Contributions are welcome! Please ensure:
1. Code follows Rust conventions
2. All tests pass
3. Code is properly formatted with `cargo fmt`
4. No clippy warnings are present

## ℹ️ Version

Current Version: 0.1.0
