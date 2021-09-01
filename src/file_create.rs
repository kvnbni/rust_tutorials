use std::fs::File; //fs -> file system manipulation operations
use std::io::prelude::*; //The purpose of this module is to alleviate imports of many common I/O
                         //traits by adding a glob import to the top of I/O heavy modules.


pub fn run(){
    //Creating file
    let mut file= File::create("test.txt").expect("Could not create file!!");
    //Using expect instead of unwrap and providing good error messages can convey your intent and
    //make tracking down the source of a panic easier. Because this error message starts with the
    //text we specified... it will be easier to find where in the code this error message is coming
    //from.

    //Writing to the file
    file.write_all(b"Hello file!!").expect("Cannot write to file"); //Writing a byte slice to the file.

}
