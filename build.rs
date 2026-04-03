use chrono::NaiveDate;
use pulldown_cmark::{Parser, html};
use serde::Serialize;

use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct PostData {
    meta: MetaData,
    html: String,
}

#[derive(Debug, Default, PartialEq, Eq, Serialize)]
struct MetaData {
    title: String,
    date: NaiveDate,
    summary: String,
}

impl Ord for MetaData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date
            .cmp(&other.date)
            .then_with(|| self.title.cmp(&other.title))
    }
}

impl PartialOrd for MetaData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let post_dir = "public/posts";
    let mut posts = vec![];

    for entry in fs::read_dir(post_dir).expect("Failed to find posts") {
        let entry = entry.expect("Failed to read post");
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).expect("Failed to read File contents");
            let data = extract_data(&content).expect("Failed to read date");
            posts.push(data);
        }
    }

    posts.sort_by(Ord::cmp);
    posts.reverse();

    let json = serde_json::to_string(&posts).expect("Failed to Serialize posts");

    let path = Path::new("public/posts.json");
    if path.exists() {
        println!("Deleting {}", path.to_str().unwrap());
        let _ = fs::remove_file(path);
    }
    fs::write("public/posts.json", json).expect("Failed to write index.json");

    println!("cargo:rerun-if-changed=public/posts/");
}

// Extracts a comment from the file, ie , <!-- title: My Title -->
fn extract_comment(content: &str, key: &str) -> Option<String> {
    let prefix = format!("<!-- {}: ", key);
    let suffix = "-->".to_string();
    content
        .lines()
        .find(|line| line.starts_with(&prefix))?
        .trim_start_matches(&prefix)
        .trim_end_matches(&suffix)
        .trim()
        .to_string()
        .into()
}

fn extract_data(content: &str) -> chrono::ParseResult<PostData> {
    let title = extract_comment(content, "title").unwrap_or_default();
    let str_date = extract_comment(content, "date").unwrap_or_default();
    let summary = extract_comment(content, "summary").unwrap_or_default();
    let data = content
        .lines()
        .filter(|line| !line.contains("<!--"))
        .collect::<Vec<_>>()
        .join("\n");

    let date = NaiveDate::parse_from_str(&str_date, "%Y-%m-%d")?;
    let parser = Parser::new(&data);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    let html = format!(
        r#"
            <h1>{}</h1><p>{}</p><h3>{}</h3><br />{}"#,
        title, date, summary, body
    );

    let meta = MetaData {
        title,
        date,
        summary,
    };

    Ok(PostData { meta, html })
}
