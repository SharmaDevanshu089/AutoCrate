use std::{fs, path};
use std::path::Path;

const CONFIG_ROOT: &str = "%APPDATA%/AutoCrate/";
pub fn is_there_config() -> bool{
    let root_path = Path::new(CONFIG_ROOT);
    let is_root_exist = if root_path.exists() 
    { 
        true
    } else {
        false
    };
    return is_root_exist;
}

pub fn test(){
    println!("Testing Sucess!");
}