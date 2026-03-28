use std::fs;

fn main() {
    let post_dir = "public/posts";
    let mut posts = vec![];

    for entry in fs::read_dir(post_dir).expect("Failed to find posts") {
        let entry = entry.expect("Failed to read post");
        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let filename = path
            .file_stem()
            .expect("Failed to Read File Stem")
            .to_str()
            .expect("Failed Stem to str")
            .to_string();

        let content = fs::read_to_string(&path).expect("Failed to read File contents");
        let title =
            extract_comment(&content, "title").unwrap_or_else(|| filename.replace('-', " "));
        let date = extract_comment(&content, "date").unwrap_or_default();
        let summary = extract_comment(&content, "summary").unwrap_or_default();

        posts.push(format!(
            r#"{{"slug":"{}","title":"{}","date":"{}", "summary","{}"}}"#,
            filename, title, date, summary
        ));
    }

    posts.sort_by(|a, b| b.cmp(a));
    let json = format!("[{}]", posts.join(","));
    fs::write("public/posts/index.json", json).expect("Failed to write index.json");

    println!("cargo:rerun-if-changed=public/posts");
}

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
