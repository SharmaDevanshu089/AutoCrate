use colored::Colorize;

use crate::{error_handler, get_input, main};

const NAME_BANNER:&str = " █████╗ ██╗   ██╗████████╗ ██████╗  ██████╗ ██████╗  █████╗ ████████╗███████╗
██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗██╔════╝ ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝
███████║██║   ██║   ██║   ██║   ██║██║      █████╔╝ ███████║   ██║   █████╗  
██╔══██║██║   ██║   ██║   ██║   ██║██║      ██╔══██╗██╔══██║   ██║   ██╔══╝  
██║  ██║╚██████╔╝   ██║   ╚██████╔╝╚██████╔╝██║  ██║██║  ██║   ██║   ███████╗
╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝";
const BARRIER:&str = "════════════════════════════════════════════════════════════════════════════════";



//this is like setup for the code
pub fn setup(){
    
}
pub fn print_intro(){
    println!("\n\n\n{}",NAME_BANNER.color("green"));
    println!("\n\n\n{}",BARRIER.color("green"));
    println!("{} is a Rust-powered CLI tool that automates your workflow by creating sequential Rust projects via a usique name, initializing git repositories, as well as automatically adding various functions as boilerplate and opening them directly in your editor to save your Precious time.\n\n  \n{}","Autocrate".bold().color("red"),BARRIER.color("green"));
    get_program_name_to_json();
}
pub fn get_program_name_to_json(){
    println!("\n\n\n\nPlease tell what will be the name of your sequential projects (eg: leet_code_question, question , test) :");
    let name = get_input().trim();
    //filtering
    
}