use std::path::Path;

pub fn setup(name: &String, path: &Path) {
    println!("Arguments passed to setup function: {} {}", name, path.display());
}