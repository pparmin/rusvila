use std::path::Path;
use std::fs;

pub fn setup(name: &String, path: &Path) {
    let root_dir = format!("{}/{}", path.display(), name);
    println!("Creating root directory for project {} in the following directory: {}", name, path.display());
    println!("Root path: {}", root_dir);
    fs::create_dir(Path::new(&root_dir)).expect("Error while creating root directory");
    let pages_dir = format!("{}/pages", root_dir);
    let static_dir = format!("{}/static", root_dir);
    let templates_dir = format!("{}/templates", root_dir);
    fs::create_dir(pages_dir).expect("Error while creating pages directory");
    fs::create_dir(static_dir).expect("Error while creating static directory");
    fs::create_dir(templates_dir).expect("Error while creating templates directory");
}