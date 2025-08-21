pub fn errorout(error: &str) {
    match error {
        "config_creation" => { println!("Unable to Create Config");}
        _ => {println!("Unkown Error Occured");}
    }
}