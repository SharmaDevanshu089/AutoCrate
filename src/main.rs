use std::io;
use colored::Colorize;
use regex::Regex;
use rfd::FileDialog;

mod config_manager;
mod error_handler;
mod first_run;


const SERIOUS_ERROR: &str = "A very Serious Internal Compilation time error occured cannot continue, try recompling from source";

fn main(){
    select_folder();
}
pub fn get_input() -> String {
    let line_read = io::read_to_string(io::stdin());
    match line_read {
        Ok(line_read) => line_read,
        Err(line_read) => {error_handler::errorout("need_string_valid", line_read.to_string());unreachable!()},
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