use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;

#[allow(unused_imports)]
use error_chain::error_chain;
// use crate::List::{Cons, Nil};

#[allow(unused_imports)]
use crate::readers::{get_request, read_from_file_by_csv, write_to_json};
use crate::readers::get_parent_recursively;
#[allow(unused_imports)]
use crate::readers::read_from_json;
use crate::structs::{Cat, EPost, MyBox, Node, NPost, Post};

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

// fn main() {//🙂
//     match read_md_files() {
//         Ok(files) => {
//             let (sender, receiver) = mpsc::channel();
//             let handlers = convert_md_to_html_parallel(files, sender);
//
//             for (index, handler) in handlers.into_iter().enumerate() {
//                 match handler.join() {
//                     Ok(_) => println!("Process {index} is finished!"),
//                     Err(error) => {
//                         if let Some(s) = error.downcast_ref::<String>() {
//                             println!("Thread {index} error occured by {s} !");
//                         } else {
//                             println!("Unknown error at thread {index}!");
//                         }
//                     }
//                 }
//             }
//
//             for received_msg in receiver {
//                 match received_msg {
//                     ProcessorMessage::Success(s) => println!("Success {s}"),
//                     ProcessorMessage::Error(e) => println!("Error {e}")
//                 }
//             }
//         }
//         Err(error) => eprintln!("Something wrong with collecting MD files: {}", error)
//     }
// }

// fn main() {
//     let mut elder_cat = Rc::new(RefCell::new(Cat { name: "Mars".to_string(), age: 15, parent: None }));
//     println!("elder_cat: {:?}", elder_cat);
//     let parent_cat = Rc::new(RefCell::new(Cat { name: "Mr.Johnson".to_string(), age: 9, parent: Some(Rc::clone(&elder_cat)) }));
//     println!("young_cat: {:?}", parent_cat);
//     let young_cat = Rc::new(RefCell::new(Cat { name: "Bobby".to_string(), age: 3, parent: Some(Rc::clone(&parent_cat)) }));
//     println!("young_cat: {:?}", young_cat);
//
//     elder_cat.borrow_mut().age = 1000;
//
//     // println!("elder_cat: {:?}", elder_cat);
//     // println!("young_cat: {:?}", parent_cat);
//     // println!("young_cat: {:?}", young_cat);
//
//     get_parent_recursively(&young_cat);
//
//     println!("======================");
//
//     let arg = 42;
//     let res = MyBox::new(&arg);
//     res.print_field();
//     println!("deref arg: {}", res.deref());
//     drop(res);
//
//     let x = 5;
//     let y = &x;
//     let z = Box::new(x);
//     let o = MyBox::new(x);
//     let u = &o;
//     let r = o.deref();
//     let t = *o;
//
//     println!("5 == x is {}", 5 == x);
//     println!("5 == *y is {}", 5 == *y);
//     println!("5 == *z is {}", 5 == *z);
//     println!("5 == *o is {}", 5 == *o);
//     println!("5 == *u is {}", 5 == *(*u).deref());
//     println!("5 == *r is {}", 5 == *r);
//
//     let mb2 = MyBox::new(String::from("World"));
//     MyBox::<String>::hello(&mb2);
// }

// fn main() {
//     let mut data1 = Arc::new(Mutex::new(1));
//     let mut data2 = Arc::new(Mutex::new(2));
//
//     let data1_clone = Arc::clone(&data1);
//     let data2_clone = Arc::clone(&data2);
//
//     let h1 = thread::spawn(move || {
//         println!("lock data2 in h1");
//         let guard2 = data2_clone.lock().unwrap();
//         println!("wait second in h1");
//         thread::sleep(std::time::Duration::from_secs(1));
//         println!("lock data1 in h1");
//         let guard1 = data1_clone.lock().unwrap();
//         println!("h1 finished!");
//     });
//
//     let h2 = thread::spawn(move || {
//         println!("lock data1 in h2");
//         let guard1 = data1.lock().unwrap();
//         println!("wait second in h2");
//         thread::sleep(std::time::Duration::from_secs(1));
//         println!("lock data2 in h2");
//         let guard2 = data2.lock().unwrap();
//         println!("h2 finished!");
//     });
//
//     h1.join().expect("Error to run h1");
//     h2.join().expect("Error to run h2");
// }

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }
//
// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }
//
// fn main() {
//     let mut elder_cat = Rc::new(RefCell::new(Cat { name: "Mars".to_string(), age: 15, parent: None }));
//     let parent_cat = Rc::new(RefCell::new(Cat { name: "Mr.Johnson".to_string(), age: 9, parent: Some(Rc::clone(&elder_cat)) }));
//     elder_cat.borrow_mut().parent = Some(Rc::clone(&parent_cat));
//
//     // println!("elder_cat: {:?}", elder_cat);
//     // println!("young_cat: {:?}", parent_cat);
//
//     let leaf = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });
//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
//
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     {
//         let branch = Rc::new(Node {
//             value: 5,
//             parent: RefCell::new(Weak::new()),
//             children: RefCell::new(vec![Rc::clone(&leaf)]),
//         });
//
//         *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
//
//         println!(
//             "branch strong = {}, weak = {}",
//             Rc::strong_count(&branch),
//             Rc::weak_count(&branch),
//         );
//
//         println!(
//             "leaf strong = {}, weak = {}",
//             Rc::strong_count(&leaf),
//             Rc::weak_count(&leaf),
//         );
//     }
//
//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
//     println!(
//         "leaf strong = {}, weak = {}",
//         Rc::strong_count(&leaf),
//         Rc::weak_count(&leaf),
//     );
//     println!("============");
//
//     let mut post = Post::new();
//     post.add_text("hello!");
//     println!("post: {}", post.content());
//     post.request_review();
//     println!("post: {}", post.content());
//     post.reject();
//     println!("post: {}", post.content());
//     post.request_review();
//     println!("post: {}", post.content());
//     post.approve();
//     println!("post: {}", post.content());
//     post.request_review();
//     println!("post: {}", post.content());
//     println!("============");
//
//     let mut npost = NPost::new();
//     npost.add_text("Hello, World!");
//     // println!("npost: {}", npost.content());
//     let npost = npost.request_review();
//     // println!("npost: {}", npost.content());
//     let npost = npost.approve();
//     println!("npost: {}", npost.content());
//     println!("============");
//
// }

fn main() {
    let mut post = EPost::new();
    post.add_text("Hello, world!!");
    post.request_review();
    post.reject();
    post.request_review();
    post.add_text("#42#");
    post.approve();
    post.approve();
    println!("Post content: {}", post.content());
}