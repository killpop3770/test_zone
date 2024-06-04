#[allow(unused_imports)]
use error_chain::error_chain;

use crate::converters::converter_celcius_and_fahrenheit;
#[allow(unused_imports)]
use crate::readers::{get_request, read_from_file_by_csv, write_to_json};
#[allow(unused_imports)]
use crate::readers::read_from_json;

mod readers;
mod structs;
mod converters;

// fn main() {
//     println!("Hello, world!");
//
//     if let Err(error) = read_from_file_by_csv("biostats.csv") {
//         eprintln!("Something wrong to read file: {}", error);
//     }
//
//     let raw_json = r#"{"article":"some_article","author":"me","paragraph":[{"name":"first"},{"name":"second"},{"name":"third"}]}"#;
//
//     let parsed_json = read_from_json(raw_json);
//     match parsed_json {
//         Ok(parsed) => println!("json to obj: {:?}", parsed),
//         Err(error) => eprintln!("Something wrong when parsed json {}", error)
//     }
//
//     let article = Article {
//         article: String::from("Main"),
//         author: String::from("Me"),
//         paragraph: vec![
//             Paragraph {
//                 name: String::from("first")
//             }
//         ],
//     };
//     let json = write_to_json(&article);
//     println!("obj to json: {}", json);
//
//     match get_request("https://httpbin.org/get") {
//         Ok(resp) => println!("response: {:?}", resp.status()),
//         Err(err) => eprintln!("request error: {}", err)
//     }
//
//     println!("Program end!");
// }
// use

// error_chain!(foreign_links{
//     Io(std::io::Error);
//     HttpRequest(reqwest::Error);
// } );
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     let result = reqwest::get("https://httpbin.org/get").await?;
//     println!("status code: {}", result.status());
//     println!("headers: {:?}", result.headers());
//     let body = result.text().await?;
//     println!("body: {}", body);
//
//     Ok(())
// }


// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
//                       owner = "rust-lang-nursery",
//                       repo = "rust-cookbook"
//     );
//     println!("{}", url);
//
//     let client = reqwest::Client::new();
//     let response = client
//         .get(&url)
//         .header(USER_AGENT, "api info")
//         .send()
//         .await?;
//
//     let users :Vec<User> = response.json().await?;
//
//     println!("{:?}", users);
//
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let url = "https://httpbin.org/get".to_string();
//     let user = "testuser".to_string();
//     let passwd: Option<String> = None;
//
//     let client = reqwest::Client::new();
//     let response = client.get(url).basic_auth(user, passwd).send().await?;
//
//
//     println!("{:?}", response);
//
//     Ok(())
// }

fn main() {

    match converter_celcius_and_fahrenheit() {
        Ok(result) => {
            println!("Result: {}", ((result * 100.) as i32) as f32 / 100.)
        }
        Err(error) => { eprintln!("Error occured by: {}", error) }
    }
}