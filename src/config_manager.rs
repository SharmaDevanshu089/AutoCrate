use std::fs::{create_dir_all, File};
use std::path::Path;
use directories::{ProjectDirs,UserDirs};
use crate::{error_handler, first_run};

const SERIOUS_ERROR: &str = "A very Serious Internal Compilation time error occured cannot continue, try recompling from source";

pub fn is_there_config() -> bool{
    let dir =get_directory("config_root");
    let root_path = Path::new(&dir);
    let is_root_exist = if root_path.exists() 
    { 
        true
    } else {
        false
    };
    return is_root_exist;
}
// pub fn get_config(){
//     if is_there_config() {
//         println!("config is there")
//     }
//     else {
//         create_root();
//     }
// }
pub fn create_root() {
    let _new_root = create_dir_all(get_directory("config_root"));
    match _new_root {
        Ok(_new_root) => (),
        Err(_new_root) => error_handler::errorout("dir_creation",_new_root.to_string()),
    }
    let _new_file = File::create(get_directory("config_file"));
    match  _new_file{
        Ok(_new_file) => (),
        Err(_new_file) => error_handler::errorout("config_creation",_new_file.to_string()),
    }
    //Config Creation Sucess but this is first run
    // first_run::setup();
}
pub fn get_directory(type_of:&str) -> String{
    let program_name = ProjectDirs::from("dev", "sharmadevanshu089", "autocrate").expect(SERIOUS_ERROR);
    let user_name=UserDirs::new().expect(SERIOUS_ERROR);
    let user_documents=user_name.document_dir().expect(SERIOUS_ERROR);
    let config_root = program_name.config_dir().to_str().expect(SERIOUS_ERROR);
    let mut config_file = program_name.config_dir().to_path_buf();
    config_file.push("_Config.json");
    // let config_file_old = config_root.to_owned()+"\\_Config.json";
    // let config_file = config_file_old.as_str();
    let mut empty_value = "";
    match type_of {
        "config_file" => empty_value = config_file.to_str().expect(SERIOUS_ERROR),
        "config_root" => empty_value = config_root,
        "documents_dir" => empty_value = user_documents.to_str().expect(SERIOUS_ERROR),
        _ => error_handler::errorout_no_log("unmentioned_type_of"),
    }
    let return_value = empty_value.to_string();
    return return_value;
}