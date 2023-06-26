use std::path::{Path, PathBuf};
use std::fs::{File};
use std::io::Write;

use handlebars::{Handlebars};
use markdown;
use serde::{Serialize};
use serde_json;

static INDEX_MD: &str = "<!-- template: homepage -->
# Hello World
";

static MAIN_JS: &str = "console.log('hello world')
";

static DEFAULT: &str = "<html>
<body>
<div id='page-content'></div>
</body>
</html>
";

static HOMEPAGE: &str = "<html>
<body>
<p>My first Teeny page</p>
<div id='page-content'></div>
<script src='main.js' />
</body>
</html>
";

#[derive(Serialize)]
pub struct Blog {
    pub name: String,
    pub location: PathBuf,
}

impl Blog {
    pub fn new() -> Self {
        Blog {
            name: String::new(),
            location: std::env::current_dir().unwrap(),
        }
    }

    fn write_config(&self) {
        let blog_json = serde_json::to_string(self).unwrap();
        println!("JSON DATA: {}", blog_json);
        let config_location = format!("{}/config.json", self.location.display().to_string().trim_end_matches("/"));
        println!("created config file at {}", config_location);

        let mut config = File::create(&config_location).expect("Error while creating project config file");
        config.write(blog_json.as_bytes()).unwrap();
        println!("written config to file");
    }


pub fn setup(&mut self, name: &String, path: &Path) {
    self.name = name.to_string();
    let mut reg = Handlebars::new();

    let root_dir = format!(
        "{}/{}",
        path.display().to_string().trim_end_matches("/"),
        self.name
    );

    self.location = root_dir.clone().into();

    println!(
        "Creating root directory for project {} in the following directory: {}",
        self.name,
        self.location.display()
    );

    println!("Root path: {}", root_dir);
    std::fs::create_dir_all(Path::new(&root_dir)).expect("Error while creating root directory");

    let pages_dir = format!("{}/pages", root_dir);
    std::fs::create_dir(&pages_dir).expect("Error while creating pages directory");

    let index_file = format!("{}/index.md", &pages_dir);
    std::fs::write(index_file, INDEX_MD).expect("error while creating index.md in pages directory");

    let static_dir = format!("{}/static", root_dir);
    std::fs::create_dir(&static_dir).expect("Error while creating static directory");
   
    let main_js = format!("{}/main.js", &static_dir);
    std::fs::write(main_js, MAIN_JS).expect("error while creating main.js in static directory");

    let templates_dir = format!("{}/templates", root_dir);
    std::fs::create_dir(&templates_dir).expect("Error while creating templates directory");

    let default_html = format!("{}/default.html", &templates_dir);
    let homepage_html = format!("{}/homepage.html", &templates_dir);
    std::fs::write(default_html, DEFAULT)
        .expect("error while creating default.html in templates directory");
    std::fs::write(homepage_html, HOMEPAGE)
        .expect("error while creating homepage.html in templates directory");

    reg.register_templates_directory(".html", templates_dir)
        .expect("error while registering templates/ directory");

    assert!(
        reg.has_template("default"),
        "default.html template not found in templates/"
    );

    assert!(
        reg.has_template("homepage"),
        "homepage.html template not found in templates/"
    );

    self.write_config()
}

pub fn build(&self) {
    println!("{}", markdown::to_html(INDEX_MD))
    
}

pub fn remove(&self) {
    //std::fs::remove_dir_all(project_root.clone()).expect("Error while removing project location");

    println!("project at {}", self.location.display())
}
}
