mod readers;

use crate::readers::{Article, get_request, Paragraph, read_from_file_by_csv, write_to_json};
use crate::readers::read_from_json;


fn main() {
    println!("Hello, world!");

    if let Err(error) = read_from_file_by_csv("biostats.csv") {
        eprintln!("Something wrong to read file: {}", error);
    }

    let raw_json = r#"
    {
        "article": "some_article",
        "author": "me",
        "paragraph": [
            {
                "name": "first"
            },
            {
                "name": "second"
            },
            {
                "name": "third"
            }
        ]
    }"#;

    let parsed_json = read_from_json(raw_json);
    match parsed_json {
        Ok(parsed) => println!("json to obj: {:?}", parsed),
        Err(error) => eprintln!("Something wrong when parsed json {}", error)
    }

    let article = Article {
        article: String::from("Main"),
        author: String::from("Me"),
        paragraph: vec![
            Paragraph {
                name: String::from("first")
            }
        ],
    };
    let json = write_to_json(&article);
    println!("obj to json: {}", json);

    match get_request("https://httpbin.org/get") {
        Ok(resp) => println!("response: {:?}", resp.status()),
        Err(err) => eprintln!("request error: {}", err)
    }

    println!("Program end!");
}
