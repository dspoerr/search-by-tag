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
extern crate tantivy;
use walkdir::WalkDir;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tantivy::doc;
use std::path::Path;


fn main() -> tantivy::Result<()> {
    let mut output = std::fs::File::create("list-of-files.txt").expect("Couldn't create file!");

    let index_path = tantivy::directory::MmapDirectory::open("C:\\Users\\dawns\\Documents\\rust\\search-by-tag\\index")?;

    let mut schema_builder = Schema::builder();
    
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let body = schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    let index = Index::open_or_create(index_path, schema.clone())?;

    let mut index_writer = index.writer(50_000_000)?;

    for files in WalkDir::new("C:\\Users\\dawns\\Documents\\rust\\search-by-tag").into_iter().filter_map(|files| files.ok())
    {
        if files.metadata().unwrap().is_file()
        {
            index_writer.add_document(doc!(
                title => files.file_name().to_str().unwrap(),
                body => files.path().to_str().unwrap()
            ));

            /*
            output.write(files.path().display().to_string().as_bytes()).expect("Failed to write directory!");
            output.write("\n".as_bytes());
            */
            println!("{}", files.path().display());

        }
    }
    println!("Directory probe completed.\n");

    index_writer.commit()?;

    let reader = index.reader()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![title, body]);

    let mut newSearch = true;

    let exitTerm = "exit";
    while newSearch != false
    {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if line.as_str() == exitTerm
        {
            newSearch = false;
            continue;
        }

        let mut query = query_parser.parse_query(&line.as_str())?;

        let mut top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    
        for (_score, doc_address) in top_docs {
            // Retrieve the actual content of documents given its `doc_address`.
            let retrieved_doc = searcher.doc(doc_address)?;
            println!("{}", schema.to_json(&retrieved_doc));
        }    
    }
    Ok(())
}
