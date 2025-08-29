use std::{fmt::format, io, path::{self, PathBuf}};
use colored::Colorize;
use regex::Regex;
use rfd::FileDialog;
use std::process::Command;
use crate::{config_manager::{get_data_from_json, get_directory}, error_handler::errorout};
use crate::first_run::BARRIER;

mod config_manager;
mod error_handler;
mod first_run;


const SERIOUS_ERROR: &str = "A very Serious Internal Compilation time error occured cannot continue, try recompling from source";

fn main(){
    if config_manager::is_there_config() {
        start_program();
    }
    else {
        config_manager::create_root();
        first_run::setup();
    }

}
pub fn get_input() -> String {
    let mut line_read =String::new() ;
    match io::stdin().read_line(&mut line_read) {
        Ok(_) => line_read.trim().to_string(),
        Err(e) => {error_handler::errorout("need_string_valid", e.to_string());unreachable!()},
    }
}
pub fn filter(data:String) -> String{
    let regex_filter=Regex::new("^[a-zA-Z0-9_-]+$").expect(SERIOUS_ERROR)/* .unwrap_or_else(|_| error_handler::errorout("regex_error", regex_filter.to_string())) */;
    // match regex_filter {
    //      Ok(regex_filter) => regex_filter,
    //      Err(regex_filter) => {error_handler::errorout("regex_error", regex_filter.to_string());unreachable!()},
    // }
    if regex_filter.is_match(data.as_str()) {
        return data;
    }
    else {
        error_handler::not_valid_string();
        return "err".to_owned();
    }
}
pub fn select_folder() -> String{
    let folder_name = FileDialog::new().pick_folder().expect(&"Unable to get folder Name".color("red"));
    let final_name = folder_name.to_string_lossy();
    return final_name.to_string();
}
fn start_program(){
    let mut highest_number = get_highest_number();
    create_new_project(highest_number);
}
fn get_highest_number() -> i32{
    //gets highest number
    let name = get_data_from_json("name".to_string()).to_string();
    let super_f = get_data_from_json("super_f".to_string());
    let mut path_to_folder = path::Path::new(&super_f).to_path_buf();
    path_to_folder.to_path_buf().push(name.clone());
    let mut highest = 1;
    let mut path_1 = path_to_folder.clone().to_path_buf();
    path_1.push(name.clone());
    path_1 = path_1.to_path_buf();
    let path_2 = path_1.clone();
    path_to_folder = path_to_folder.to_path_buf();
    path_to_folder.push(name);
    let mut str = format!("{}{}",path_to_folder.to_string_lossy(),highest);
    path_to_folder = PathBuf::from(str);
    // println!("{}",path_1.to_string_lossy());
    // println!("{}\n{}",path_to_folder.to_string_lossy(),path_2.to_string_lossy());
    while path_to_folder.exists() {
        highest = highest+1;
        let mut str_2 = format!("{}{}",path_1.to_string_lossy(),highest);
        path_to_folder = PathBuf::from(str_2);
    }
    if highest == 0{
        highest = 1;
    }
    return highest;
}
fn create_new_project(highest:i32){
    let name = get_data_from_json("name".to_string());
    let dircetory = get_data_from_json("super_f".to_string());
    let location = format!("{}{}{}{}",dircetory,"\\",name,highest);
    let movable_location = location.clone();
    let status = Command::new("cargo").arg("new").arg(location).status().expect(SERIOUS_ERROR);
    if status.success(){
        let barrier = BARRIER.color("green");
        let program_name = format!("{}{}",name.clone(),highest.clone()).color("green").italic();
        let msg = "The Cargo Program".color("yellow");
        let msg_3 = "has been Created , would you like to open it?".color("yellow");
        let msg_2 = "Press [y] to open".bold().color("yellow") ;
        println!("{}\n{} {} {}\n{}",barrier,msg,program_name,msg_3,msg_2);
        let open = get_input();
        if open == "y"|| open == "Y"{
            open_in_editor( movable_location);
    }
    else {
        let msg = "CARGO HAS RETURNED A ERROR".bold().color("RED").on_blue();
        println!("{}",msg);
    }

    }}
fn open_in_editor(locale:String){
    let command = Command::new("C:\\Users\\YourUser\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe").arg(locale).output();
    match command {
       Ok(command) => println!("ok"),
       Err(command) => println!("{}",command),
    }
}