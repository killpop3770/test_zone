use std::cell::RefCell;
use std::error::Error;
use std::{env, io};

use crate::structs::{Article, Cat, Paragraph};
use csv;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub fn get_parent_recursively(cat: &Rc<RefCell<Cat>>) {
    let mut current = Some(Rc::clone(cat));

    while let Some(current_cat) = current.take() {
        let current_cat_ref = current_cat.borrow();
        println!(
            "Cat with name {} and age {} ",
            current_cat_ref.name, current_cat_ref.age
        );

        if let Some(current_cat_parent_ref) = current_cat_ref.parent.as_ref() {
            let current_cat_parent = current_cat_parent_ref.borrow();
            println!("has parent with name {} !", current_cat_parent.name);
            current = Some(Rc::clone(&current_cat_parent_ref));
        } else {
            println!("has no parents !");
        }
    }
}

pub fn read_md_files() -> Result<Vec<PathBuf>, io::Error> {
    let mut md_files_dir = env::current_dir()?;
    md_files_dir.push("../md");
    let entires = fs::read_dir(md_files_dir)?;

    let mut path_vec = vec![];

    for entire in entires {
        path_vec.push(entire?.path());
    }

    Ok(path_vec)
}

pub fn write_to_json(obj: &Article) -> String {
    let json = serde_json::to_string(obj).unwrap();
    json
}

pub fn read_from_file_by_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut data = csv::Reader::from_path(file_path)?;

    for line in data.records() {
        match line {
            Ok(res) => println!("{:?}", res),
            Err(err) => eprintln!("Error by parsing csv: {:?}", err),
        }
    }

    Ok(())
}

pub fn read_from_json(json: &str) -> Result<Article, Box<dyn Error>> {
    let parsed_json = serde_json::from_str(json)?;

    Ok(parsed_json)
}

pub fn get_request(addr: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let response = reqwest::blocking::get(addr)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_file_by_csv_bad_path() {
        assert!(read_from_file_by_csv("123").is_err());
    }

    #[test]
    fn test_read_from_file_by_csv() {
        assert!(read_from_file_by_csv("biostats.csv").is_ok());
    }

    #[test]
    fn test_read_from_json() {
        let raw_json = r#"{
        "article": "some_article",
        "author": "me",
        "paragraph": [{"name": "first"},{"name": "second"},{"name": "third"}]}"#;

        let res = read_from_json(raw_json).unwrap();
        assert_eq!("third", res.paragraph[2].name);
    }

    #[test]
    fn test_read_from_json_bad_data() {
        let raw_json = r#"
    {
        "article": "some_article",
        "author": "me...
    }"#;

        assert!(read_from_json(raw_json).is_err());
    }

    #[test]
    fn test_write_to_json() {
        let article = Article {
            article: String::from("Main"),
            author: String::from("Me"),
            paragraph: vec![Paragraph {
                name: String::from("first"),
            }],
        };
        let target = r#"{"article":"Main","author":"Me","paragraph":[{"name":"first"}]}"#;
        assert_eq!(target, write_to_json(&article));
    }
}
