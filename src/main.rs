/*
 * References -
 * https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
 * https://natclark.com/tutorials/rust-list-all-files/
 *
 * Possible things to add long term:
 * Parse directory and add tags based on the name of the file or location it's in
 * Windows right click GUI that lets you add a tag to a file for easy access
 *
 * Current TODO:
 * Send output to a database file of some sort
 * Enable searching the database file for name
 * Add a section in the database for tags. enable searching via tags
 */ 

use std::io::Write;

extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    let mut output = std::fs::File::create("list-of-files.txt").expect("Couldn't create file!");

    for files in WalkDir::new("C:\\").into_iter().filter_map(|files| files.ok())
    {
        if files.metadata().unwrap().is_file()
        {
            output.write(files.path().display().to_string().as_bytes()).expect("Failed to write directory!");
            output.write("\n".as_bytes());
            println!("{}", files.path().display());
        }
    }

    println!("Directory probe completed.\n");
}
