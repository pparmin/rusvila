use std::path::Path;
//use std::fs::{File, create_dir_all};

static INDEX_MD: &str = 
"<!-- template: homepage -->
# Hello World
";

static MAIN_JS: &str = 
"console.log('hello world')
";

static DEFAULT: &str = 
"<html>
  <body>
    <div id='page-content'></div>
  </body>
</html>
";

static HOMEPAGE: &str = 
"<html>
  <body>
    <p>My first Teeny page</p>
    <div id='page-content'></div>
    <script src='main.js' />
  </body>
</html>
";


pub fn setup(name: &String, path: &Path) {
    let root_dir = format!("{}/{}", path.display(), name);
    println!("Creating root directory for project {} in the following directory: {}", name, path.display());
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
    std::fs::write(default_html, DEFAULT).expect("error while creating default.html in templates directory");
    std::fs::write(homepage_html, HOMEPAGE).expect("error while creating homepage.html in templates directory");
}