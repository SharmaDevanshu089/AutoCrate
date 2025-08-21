use std::fs::File;
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
    let newfile = File::create(CONFIG_FILE);
    match  newfile{
        Ok(_) => (),
        Err(_) => error_handler::errorout("config_creation"),
    }
}