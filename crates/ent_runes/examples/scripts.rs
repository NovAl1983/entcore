use std::collections::HashMap;
use std::fs;
use std::io;
use std::println;
use std::path::{Path, PathBuf};
use std::str::FromStr;

//let script_source =  //исходный код скрипта (строка)

fn main()-> Result<(), Box<dyn std::error::Error>> {
    println!("Hello!!!");
    let mut scripts: HashMap<String, Script> = HashMap::new();
    let dir = PathBuf::from_str(r"D:\Rust\projects\entcore\rune_scripts");
    let script_paths = collect_rn_files(dir?)?;
    for script in script_paths {
        let name: String = script.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let path = script.to_string_lossy().into_owned();
        let file = fs::File::open(path)?;
        let source: Result<String, io::Error> = io::read_to_string(file);
        scripts.insert(name.clone(), Script { name: name.clone(), path: script, source_code: source });
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
    pub source_code: Result<String, io::Error>
}

impl Script {
    pub fn new(name: String, path: PathBuf, source_code: Result<String, io::Error> ) -> Self {
        Self { name, path, source_code }
    }
}

