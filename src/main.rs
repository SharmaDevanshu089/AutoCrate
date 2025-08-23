use std::io;

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
