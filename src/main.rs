use std::sync::mpsc;

#[allow(unused_imports)]
use error_chain::error_chain;

use crate::converters::convert_md_to_html_parallel;
#[allow(unused_imports)]
use crate::readers::{get_request, read_from_file_by_csv, write_to_json};
#[allow(unused_imports)]
use crate::readers::read_from_json;
use crate::readers::read_md_files;
use crate::structs::ProcessorMessage;

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

// fn main() {
//
//     match converter_celcius_and_fahrenheit() {
//         Ok(result) => {
//             println!("Result: {}", ((result * 100.) as i32) as f32 / 100.)
//         }
//         Err(error) => { eprintln!("Error occured by: {}", error) }
//     }
// }

// fn main() {
//     let handler = thread::spawn(|| {
//         println!("hi!");
//         println!("{:?}", thread::current().id());
//         thread::sleep(time::Duration::from_millis(1));
//     });
//     thread::spawn(|| {
//         for i in 0..10 {
//             println!("hi #{} from spawn thread: {:?}", i, thread::current().id());
//             thread::sleep(time::Duration::from_millis(5));
//         }
//     });
//
//     for i in 0..10 {
//         println!("hi #{} from main thread: {:?}", i, thread::current().id());
//         thread::sleep(time::Duration::from_millis(5));
//     }
//     handler.join().expect("Error with thread!");
//
//     let (sender, receiver) = mpsc::channel();
//
//     let another_sender = sender.clone();
//
//     thread::spawn(move || {
//         let msg = "Hi";
//         for i in 1..=5 {
//             let res = format!("{}#{} from {:?} !", msg, i, thread::current().id());
//             sender.send(res).unwrap();
//             thread::sleep(Duration::from_secs(2));
//         }
//     });
//
//     thread::spawn(move || {
//         let msg = "Bye";
//         for i in 1..=5 {
//             let res = format!("{}#{} from {:?} !", msg, i, thread::current().id());
//             another_sender.send(res).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received_data in receiver {
//         println!("{}", received_data);
//     }
//
//     println!("===================");
//
//     let data = Arc::new(Mutex::new(10));
//     let mut handlers = vec![];
//
//     for _ in 0..10 {
//         let data_ref = Arc::clone(&data);
//
//         let handler = thread::spawn(move || {
//             let mut deref_data = data_ref.lock().unwrap();
//             *deref_data += 1;
//             println!("{} from {:?}", *deref_data, thread::current().id());
//         });
//
//         handlers.push(handler)
//     }
//
//     for handler in handlers {
//         drop(handler);
//     }
// }

fn main() {//🙂
    match read_md_files() {
        Ok(files) => {
            let (sender, receiver) = mpsc::channel();
            let handlers = convert_md_to_html_parallel(files, sender);

            for (index, handler) in handlers.into_iter().enumerate() {
                match handler.join() {
                    Ok(_) => println!("Process {index} is finished!"),
                    Err(error) => {
                        if let Some(s) = error.downcast_ref::<String>() {
                            println!("Thread {index} error occured by {s} !");
                        } else {
                            println!("Unknown error at thread {index}!");
                        }
                    }
                }
            }

            for received_msg in receiver {
                match received_msg {
                    ProcessorMessage::Success(s) => println!("Success {s}"),
                    ProcessorMessage::Error(e) => println!("Error {e}")
                }
            }
        }
        Err(error) => eprintln!("Something wrong with collecting MD files: {}", error)
    }
}