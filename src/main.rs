/*
 * References -
 * https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
 * https://natclark.com/tutorials/rust-list-all-files/
 *
 */ 

use std::io::Write;
use std::fs;

fn main() {
    let mut file = std::fs::File::create("test.txt").expect("Couldn't create file! :(");
    
    let testdata = "Test data!\n Testing\n";

    file.write_all("Testing...".as_bytes()).expect("Failed to write!");
    file.write(testdata.as_bytes()).expect("Failed to write!");
    println!("Hello, world!");

    for files in fs::read_dir("C:\\").unwrap() {
       file.write(files.unwrap().path().display().to_string().as_bytes()).expect("Failed to write directory!");
       file.write("\n".as_bytes());
    }
}
