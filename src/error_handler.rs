use colored::Colorize;
use std::fs::OpenOptions;
use std::io::Write;

use crate::{
    first_run::{self, get_program_name_to_json},
    main,
};

const LOGFILE: &str = "autocrate_debug.log";

pub fn errorout(error: &str, to_log: String) {
    match error {
        "config_creation" => {
            println!("Unable to Create Config;");
            log(to_log);
        }
        "dir_creation" => {
            println!("Unable to Create Root Directory;");
            log(to_log);
        }
        "need_string_valid" => {
            println!("Need Valid String");
            first_run::get_program_name_to_json();
        }
        "name_is_empty" => {
            println!("The String should not be empty");
            first_run::get_program_name_to_json();
        }
        "regex_error" => {
            println!(
                "Unable to Create a Filter. This is a fatal error. Please check github for latest Release. Restarting Program"
            );
            main();
            log(to_log);
        }
        "folder_diag" => {
            println!("Unable to get folder. Try Again");
            first_run::get_program_name_to_json();
            log(to_log);
        }
        _ => {
            println!("Unkown Error Occured");
            panic!("Error Occured")
        }
    }
}
fn log(to_log: String) {
    let mut log_file = OpenOptions::new().append(true).create(true).open(LOGFILE).expect("We are unable to find or create a logfile this is a fatal error and we really dont know what to do;");
    log_file
        .write_all(to_log.as_bytes())
        .expect("AutoCrate is unable to write to the log file.;");
}
pub fn errorout_no_log(error: &str) {
    let to_log = "No Log Error".to_string();
    match error {
        "unmentioned_type_of" => {
            println!("The Program didnt mention the type of directory to fetch;");
            log(to_log);
        }
        _ => {
            println!("Unkown Error Occured;");
        }
    }
}
pub fn not_valid_string() {
    let error_msg = "Name Can only be a legal foldername and only can contain ".color("red");
    let highlight = "Alphabets (A-Z,a-z), Number (0-9)".bold().color("yellow");
    println!("{} {}", error_msg, highlight);
    get_program_name_to_json();
}
