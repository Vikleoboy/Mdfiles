use chrono::{DateTime, Local};
use regex::Regex;
use serde::Serialize;
use serde_yaml;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum NodeType {
    File,
    Folder,
}

#[derive(Serialize, Debug)]
struct Node {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,
}

#[derive(Serialize, Debug, serde::Deserialize, Default)]
struct Metadata {
    #[serde(default = "default_title")]
    title: String,
    #[serde(default = "default_description")]
    description: String,
    #[serde(default = "default_tags")]
    tags: Vec<String>,
    #[serde(default = "default_similar_posts")]
    similar_posts: Vec<String>,
    #[serde(default = "default_date")]
    date: String,
    #[serde(default = "default_category")]
    category: String,
    #[serde(default = "default_author")]
    author: String,
}
fn default_title() -> String { "Untitled".to_string() }
fn default_description() -> String { "No description provided.".to_string() }
fn default_tags() -> Vec<String> { vec!["general".to_string()] }
fn default_similar_posts() -> Vec<String> { Vec::new() }
fn default_date() -> String { "1970-01-01".to_string() }
fn default_category() -> String { "uncategorized".to_string() }
fn default_author() -> String { "Anonymous".to_string() }


fn extract_front_matter(path: &Path) -> Option<Metadata> {
    let content = fs::read_to_string(path).ok()?;
    let re = Regex::new(r"(?s)^---\r?\n(.*?)\r?\n---").ok()?;
    let caps = re.captures(&content)?;
    let yaml_str = caps.get(1)?.as_str();
    serde_yaml::from_str::<Metadata>(yaml_str).ok()
}

fn get_structure(path: &Path, root: &Path, ignore_list: &Vec<String>) -> Option<Node> {
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let rel_path = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    if path.is_dir() {
        let children: Vec<Node> = fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let fname = entry.file_name();
                let fname = fname.to_string_lossy();
                if ignore_list.contains(&fname.to_string()) {
                    return None;
                }

                get_structure(&entry.path(), root, ignore_list)
            })
            .collect();
        Some(Node {
            name,
            node_type: NodeType::Folder,
            path: rel_path,
            date_created: None,
            children: Some(children),
            metadata: None,
        })
    } else {
        // Get created date (fallback to modified if not available)
        let metadata = fs::metadata(path).ok();
        let created = metadata
            .and_then(|m| m.created().or_else(|_| m.modified()).ok())
            .map(|t| {
                let datetime: DateTime<Local> = DateTime::from(t);
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            });

        // Extract front matter metadata if this is a markdown file
        let file_metadata = if path.extension().map(|e| e == "md").unwrap_or(false) {
            extract_front_matter(path).unwrap_or_default()
        } else {
            Metadata::default()
        };
        Some(Node {
            name,
            node_type: NodeType::File,
            path: rel_path,
            date_created: created,
            children: None,
            metadata: Some(file_metadata),
        })
    }
}

fn main() {
    let root = std::env::current_dir().unwrap();
    let ignore_list = match fs::read_to_string("ignore_list.txt") {
        Ok(lst) => lst.lines().map(|l| l.trim().to_string()).collect(),
        Err(_) => vec![
            ".git".to_string(),
            ".DS_Store".to_string(),
            "__pycache__".to_string(),
            ".gitkeep".to_string(),
            "layout_gen".to_string(),
        ],
    };
    let structure = get_structure(&root, &root, &ignore_list);
    if let Some(structure) = structure {
        let json = serde_json::to_string_pretty(&structure).unwrap();
        fs::write(root.join("layout_rust.json"), json).unwrap();
        println!("layout_rust.json generated.");
    } else {
        println!("No structure generated.");
    }
}
