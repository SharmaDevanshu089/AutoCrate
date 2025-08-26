use colored::Colorize;

use crate::{config_manager, error_handler, get_input, main, select_folder};

const NAME_BANNER:&str = " █████╗ ██╗   ██╗████████╗ ██████╗  ██████╗ ██████╗  █████╗ ████████╗███████╗
██╔══██╗██║   ██║╚══██╔══╝██╔═══██╗██╔════╝ ██╔══██╗██╔══██╗╚══██╔══╝██╔════╝
███████║██║   ██║   ██║   ██║   ██║██║      █████╔╝ ███████║   ██║   █████╗  
██╔══██║██║   ██║   ██║   ██║   ██║██║      ██╔══██╗██╔══██║   ██║   ██╔══╝  
██║  ██║╚██████╔╝   ██║   ╚██████╔╝╚██████╔╝██║  ██║██║  ██║   ██║   ███████╗
╚═╝  ╚═╝ ╚═════╝    ╚═╝    ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝";
const BARRIER:&str = "════════════════════════════════════════════════════════════════════════════════";

    struct _Config{
        serial_name:String,
        //TODO : use path instead of string
        super_folder_path:String,
        add_shortcut:bool,
        editor:String,
    }

    //this is like setup for the code
    pub fn setup(){
        print_intro();
        get_program_name_to_json();
    }
    pub fn print_intro(){
        println!("\n\n\n{}",NAME_BANNER.color("green"));
        println!("\n\n\n{}",BARRIER.color("green"));
        println!("{} is a Rust-powered CLI tool that automates your workflow by creating sequential Rust projects via a usique name, initializing git repositories, as well as automatically adding various functions as boilerplate and opening them directly in your editor to save your Precious time.\n\n  \n{}","Autocrate".bold().color("red"),BARRIER.color("green"));
    }
    pub fn get_program_name_to_json(){
        let msg ="\n\n\n\nPlease tell what will be the name of your sequential projects (eg: leet_code_question, question , test) :".color("green").to_string();
        println!("{}",msg);
        let name = get_input().trim().to_string();
        //filtering
        let filtered_name =crate::filter(name.to_string());
        //this will initialise default json
        let mut default_config = _Config {
            serial_name : filtered_name,
            //adding default user documents as default file
            super_folder_path : config_manager::get_directory("documents_dir"),
            //adding shortcut to start
            add_shortcut : true,
            //currently only supports code
            editor : "code".to_owned(),
        };
        //added path
        default_config = get_super_path_from_user(default_config);
        
    }
    pub fn get_super_path_from_user(mut config:_Config) -> _Config{
        let msg = "\nWhat Should be your folder where files will be stored: Press [y] to choose folder".color("green");
        let barrier = BARRIER.color("yellow");
        println!("{}\n{}" ,barrier,msg);
        let choice = get_input();
        if choice == "y" ||choice == "Y" {
            config.super_folder_path = select_folder();
        }
        else {
            return get_super_path_from_user(config);
        }
        return config;
    }