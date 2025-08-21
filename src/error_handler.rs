use std::fs::OpenOptions;
use std::io::Write;

const LOGFILE:&str = "autocrate_debug.log"; 

pub fn errorout(error: &str, to_log:String) {
    match error {
        "config_creation" => { println!("Unable to Create Config");log(to_log);}
        "dir_creation" => { println!("Unable to Create Root Directory");log(to_log);}
        _ => {println!("Unkown Error Occured");}
    }
}
fn log(to_log:String) {
    let mut log_file = OpenOptions::new().append(true).create(true).open(LOGFILE).expect("We are unable to find or create a logfile this is a fatal error and we really dont know what to do");
    log_file.write_all(to_log.as_bytes()).expect("AutoCrate is unable to write to the log file.");
}