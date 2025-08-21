use std::fs::{create_dir_all, File};
//use std::{fs, path};
use std::path::Path;

use crate::{error_handler, first_run};

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
pub fn get_config(){
    if is_there_config() {
        println!("Config is there");
    }
    else {
        create_root();
    }
}
pub fn create_root() {
    let _new_root = create_dir_all(CONFIG_ROOT);
    match _new_root {
        Ok(_new_root) => (),
        Err(_new_root) => error_handler::errorout("dir_creation",_new_root.to_string()),
    }
    let _new_file = File::create(CONFIG_FILE);
    match  _new_file{
        Ok(_new_file) => (),
        Err(_new_file) => error_handler::errorout("config_creation",_new_file.to_string()),
    }
    //Config Creation Sucess but this is first run
    first_run::setup();
}