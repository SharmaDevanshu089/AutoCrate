use std::fs::{create_dir_all, File};
use std::{fs, path};
use std::path::Path;

use crate::error_handler;

const CONFIG_ROOT: &str = "DATA";
const CONFIG_FILE: &str = "DATA/_Config.json";

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
    if is_there_config() {
        println!("Config is there");
    }
    else {
        create_root();
    }
}
pub fn create_root() {
    let new_root = create_dir_all(CONFIG_ROOT);
    match new_root {
        Ok(new_root) => (),
        Err(new_root) => error_handler::errorout("dir_creation",new_root.to_string()),
    }
    let new_file = File::create(CONFIG_FILE);
    match  new_file{
        Ok(new_file) => (),
        Err(new_file) => error_handler::errorout("config_creation",new_file.to_string()),
    }
}