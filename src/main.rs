//mod file;
use std::io::Write;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
fn main() {
    let Count = Path::new("Count.txt");
    if Count.exists(){
        fnloop();
    }
    else {
        let newfile = File::create("Count.txt").unwrap();
    }
}
fn fnloop() {
    let mut i :i128 = 0;
    let mut file = OpenOptions::new().append(true).open("Count.txt").unwrap();
    while i > -1 {
    writeln!(file,"{}",i).unwrap();
    i = i+1;
    }
}