use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use cfg::Config;

//let script_source =  //исходный код скрипта (строка)

fn main()->Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello!!!");
    // let mut scripts: HashMap<String, Script> = HashMap::new();
    let mut scripts: Vec<Script> = Vec::new();

    let dir: PathBuf = { 
    let cfg: Config = Config::load_config("config.toml")?;
    PathBuf::from(&cfg.system.scripts_directory)
    };

    let script_paths: Vec<PathBuf> = collect_rn_files(dir)?;
    for script in script_paths {
        let name: String = script.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
         let source= fs::read_to_string(&script)?;
        // scripts.insert(name.clone(), Script { name: name.clone(), path: script, source_code: source });
        scripts.push(Script { 
            name: name, 
            path: script, 
            source_code: source 
        });
    }

    println!("{:?}", scripts);
    
Ok(())
}

fn collect_rn_files(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        match (path.extension(), ft.is_file()) {
            (Some(ext), true) if ext == "rn" => { result.push(path);}
            _ => {}
        }
    }
 Ok(result)   
}



#[derive(Debug)]
pub struct Script {
    pub name: String,
    pub path: PathBuf,
    pub source_code: String
}

impl Script {
    pub fn new(name: String, path: PathBuf, source_code: String  ) -> Self {
        Self { name, path, source_code }
    }
}

