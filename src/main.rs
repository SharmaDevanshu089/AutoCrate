use std::io;
use regex::Regex;

mod config_manager;
mod error_handler;
mod first_run;
fn main(){
    first_run::print_intro();
}
pub fn get_input() -> String {
    let line_read = io::read_to_string(io::stdin());
    match line_read {
        Ok(line_read) => line_read,
        Err(line_read) => {error_handler::errorout("need_string_valid", line_read.to_string());unreachable!()},
    }
}
pub fn filter(data:String) -> String{
    let regex_filter=Regex::new("^[a-zA-Z0-9_-]+$")/* .unwrap_or_else(|_| error_handler::errorout("regex_error", regex_filter.to_string())) */;
    match regex_filter {
         Ok(regex_filter) => regex_filter,
         Err(regex_filter) => error_handler::errorout("regex_error", regex_filter.to_string()),
    }
    if regex_filter.is_match(data) {
        return data;
    }
    else {
        error_handler::not_valid_string();
        return "err".to_owned();
    }
}