use pulldown_cmark::{Parser, html};
use serde_json::json;
use std::fs;
use std::path::Path;

fn main() {
    let post_dir = "public/posts";
    let mut posts = vec![];

    for entry in fs::read_dir(post_dir).expect("Failed to find posts") {
        let entry = entry.expect("Failed to read post");
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).expect("Failed to read File contents");
            let data = extract_data(&content);
            posts.push(data);
        }
    }

    //posts.sort_by(|a, b| b.cmp(a));
    let json = format!("[{}]", posts.join(","));
    let path = Path::new("public/posts.json");
    if path.exists() {
        println!("Deleting {}", path.to_str().unwrap());
        fs::remove_file(path);
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

fn extract_data(content: &str) -> String {
    let title = extract_comment(content, "title").unwrap_or_default();
    let date = extract_comment(content, "date").unwrap_or_default();
    let summary = extract_comment(content, "summary").unwrap_or_default();
    let data = content
        .lines()
        .filter(|line| !line.contains("<!--"))
        .collect::<Vec<_>>()
        .join("\n");

    let header = json!({
        "title": title,
        "date": date,
        "summary": summary,
    });

    let parser = Parser::new(&data);
    let mut body = String::new();
    html::push_html(&mut body, parser);

    let html = format!(
        r#"
            <h1>{}</h1><p>{}</p><h3>{}</h3><br />{}"#,
        title, date, summary, body
    );

    json!({
        "post_meta": header,
        "html": html,
    })
    .to_string()
}
