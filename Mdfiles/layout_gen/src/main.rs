use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum NodeType {
    File,
    Folder,
}

#[derive(Serialize)]
struct Node {
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Node>>,
}

fn get_structure(path: &Path, root: &Path) -> Node {
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    let rel_path = path
        .strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();

    if path.is_dir() {
        let children = fs::read_dir(path)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let fname = entry.file_name();
                let fname = fname.to_string_lossy();
                if fname == ".git"
                    || fname == ".DS_Store"
                    || fname == "__pycache__"
                    || fname == ".gitkeep"
                {
                    return None;
                }
                Some(get_structure(&entry.path(), root))
            })
            .collect();
        Node {
            name,
            node_type: NodeType::Folder,
            path: rel_path,
            date_created: None,
            children: Some(children),
        }
    } else {
        // Get created date (fallback to modified if not available)
        let metadata = fs::metadata(path).ok();
        let created = metadata
            .and_then(|m| m.created().or_else(|_| m.modified()).ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs().to_string());

        Node {
            name,
            node_type: NodeType::File,
            path: rel_path,
            date_created: created,
            children: None,
        }
    }
}

fn main() {
    let root = std::env::current_dir().unwrap();
    let structure = get_structure(&root, &root);
    let json = serde_json::to_string_pretty(&structure).unwrap();
    fs::write(root.join("layout_rust.json"), json).unwrap();
    println!("layout_rust.json generated.");
}
