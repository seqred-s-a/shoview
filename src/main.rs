mod view;

extern crate mongodb;
#[macro_use]
extern crate log;
extern crate argparse;
extern crate serde_json;
extern crate simplelog;
extern crate tera;

use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::{Client, Error, ThreadedClient};
use std::io::Write;

use simplelog::*;

use argparse::{ArgumentParser, Store};

fn save_to_file(filename: &String, text: &String) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(filename)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

#[test]
fn test_get_collection() {
    assert_eq!(
        get_collection(&"tmp-collection-shodan".to_string()).is_err(),
        false
    );
}

fn get_collection(collection_name: &String) -> Result<Collection, Error> {
    let client = Client::connect("localhost", 27017)?;
    Ok(client.db("test-db").collection(collection_name))
}

fn main() {
    // configure logging
    TermLogger::init(LevelFilter::Info, Config::default()).unwrap();
    info!("Start");

    let mut collection = "tmp-collection-shodan".to_string();
    let mut query_string = "".to_string();
    let mut output_file_name = "out.html".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut collection)
            .add_option(&["-c", "--collection"], Store, "MongoDB collection name (can be any string). Use separate collections for different sets of data. Uses 'tmp-collection-shodan' by default.");
        ap.refer(&mut query_string)
            .add_option(&["-q", "--query"], Store, r#"Mongo db query string (eg. -q '{"server" : "nginx"}'). By default returns all records in db (no filter). Some interesting filters are: '{ "vulns" : { "$exists" : true }}' - shows hosts with vulnerabilities; '{ "vulns" : { "$exists" : true }, "product" : "nginx"}' - all vulnerable nginx hosts. You can use any query filters as referenced in https://docs.mongodb.com/manual/reference/"#);
        ap.refer(&mut output_file_name).add_option(
            &["-o", "--output"],
            Store,
            "File to save rendered results. Uses out.html by default.",
        );
        ap.parse_args_or_exit();
    }

    // try to open db collection
    let coll = get_collection(&collection).expect("Cannot get collection");

    let find_query = match query_string.as_str() {
        "" => {
            info!("Empty query - get all docs");
            None
        }
        _ => {
            // query string
            let query: serde_json::Value = match serde_json::from_str(&query_string) {
                Ok(d) => d,
                Err(_) => {
                    error!("Invalid query string");
                    panic!();
                }
            };

            println!(
                "Your query:\n{}",
                serde_json::to_string_pretty(&query).expect("Error parsing query.")
            );

            let doc: mongodb::Bson = query.into();
            Some(
                doc.as_document()
                    .expect("Cannot parse document, change your query")
                    .clone(),
            )
        }
    };

    // print results from db
    let mut cursor = coll
        .find(find_query, None)
        .ok()
        .expect("Failed to execute find. Is mongod running?");

    // render view
    let html = view::render_view(&mut cursor);

    // save view to file
    save_to_file(&output_file_name, &html).expect("File save failed");

    info!("Completed");
}
