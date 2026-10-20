use std::io;
use std::path::{Path, PathBuf};
use std::fs;


#[derive(Debug)]
pub struct Script {
    pub name: String,
    pub path: PathBuf,
    pub source_code: String
}



pub fn  load_scripts(dir: impl AsRef<Path>) -> io::Result<Vec<Script>> {
    let mut scripts: Vec<Script> = Vec::new();

    let script_paths: Vec<PathBuf> = collect_rn_files(dir)?;

    for script in script_paths {
        let name: String = script.file_stem()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| script.to_string_lossy().into_owned());
         let source= fs::read_to_string(&script)?;
        scripts.push(Script { 
            name: name, 
            path: script, 
            source_code: source 
        });
    }
   Ok(scripts) 
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_scripts() {
        let dir = "D:\\Rust\\projects\\entcore\\rune_scripts".to_string();
        let scripts = load_scripts(dir).expect("Должно прочитаться");;
        println!("📜 Загружено {} скриптов", scripts.len());
        assert!(!scripts.is_empty());
    }
}