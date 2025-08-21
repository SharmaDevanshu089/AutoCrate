use std::fs;

pub fn write(){
    println!("{}",fs::read_to_string("readme.txt").unwrap());
}