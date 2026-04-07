use std::io::{self, Read, Write};

use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde_json::Value;

fn strip_frontmatter(content: &str) -> String {
    Matter::<YAML>::new().parse::<serde_json::Value>(content).unwrap().content
}

fn process_items(items: &mut Value) {
    let Some(arr) = items.as_array_mut() else {
        return;
    };
    for item in arr {
        let Some(chapter) = item.get_mut("Chapter") else {
            continue;
        };
        if let Some(content) = chapter.get("content").and_then(Value::as_str) {
            let stripped = strip_frontmatter(content);
            chapter["content"] = Value::String(stripped);
        }
        if let Some(sub_items) = chapter.get_mut("sub_items") {
            process_items(sub_items);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("supports") {
        std::process::exit(0);
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    let mut doc: Value = serde_json::from_str(&input).expect("parse JSON");
    let book = doc.get_mut(1).expect("book is second element");
    // NOTE: mdbook 0.5 renamed "sections" to "items" in the book JSON.
    // "sections" silently returns None — always verify against the mdbook
    // version in use if chapters appear unprocessed.
    if let Some(items) = book.get_mut("items") {
        process_items(items);
    }

    io::stdout()
        .write_all(book.to_string().as_bytes())
        .expect("write stdout");
}
