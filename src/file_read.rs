use std::fs::File;
use std::io::prelude::*;

pub fn run(){
    let mut file = File::open("test.txt").expect("Cannot read file");

    //Creating string to save file content
    let mut contents = String::new();

    ////Read the entire contents of a file into a string
    file.read_to_string(&mut contents).expect("Unable to read contents of file to string");

    println!("{}",contents );
}
