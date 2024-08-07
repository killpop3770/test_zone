use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::ops::Deref;
use std::os::fd::AsRawFd;

#[allow(unused_imports)]
use error_chain::error_chain;
use rand::Rng;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::readers::{get_request, read_from_file_by_csv, write_to_json};
#[allow(unused_imports)]
use crate::readers::read_from_json;
use crate::structs::{Animal, OutlinePrint, Pilot, Wizard};

mod converters;
mod readers;
mod structs;
mod decl_macros;

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

// struct Object {
//     state: Option<Box<dyn State>>,
// }
//
// impl Object {
//     pub fn new() -> Object {
//         Object {
//             state: Some(Box::new(Default)),
//         }
//     }
//     pub fn forward(&mut self) {
//         if let Some(x) = self.state.take() {
//             self.state = Some(x.forward());
//         }
//     }
//
//     pub fn backward(&mut self) {
//         if let Some(x) = self.state.take() {
//             self.state = Some(x.backward());
//         }
//     }
// }
//
// trait State {
//     fn forward(self: Box<Self>) -> Box<dyn State>;
//     fn backward(self: Box<Self>) -> Box<dyn State>;
// }
//
// struct Default;
//
// impl State for Default {
//     fn forward(self: Box<Self>) -> Box<dyn State> {
//         println!("forwarded!");
//         Box::new(Forwarded {})
//     }
//     fn backward(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }
//
// struct Forwarded;
//
// impl State for Forwarded {
//     fn forward(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn backward(self: Box<Self>) -> Box<dyn State> {
//         println!("backwarded!");
//         Box::new(Default {})
//     }
// }
//
// fn main() {
//     let mut post = EPost::new();
//     post.add_text("Hello, world!!");
//     post.request_review();
//     post.reject();
//     post.request_review();
//     post.add_text("#42#");
//     post.approve();
//     post.approve();
//     println!("Post content: {}", post.content());
//
//     let mut r = Object::new();
//     r.forward();
//     r.backward();
// }

// fn main() {
//     enum Color {
//         Rgb(i32, i32, i32),
//         Hsv(i32, i32, i32),
//     }
//
//     let a = Color::Rgb(1, 2, 3);
//     let res = match a {
//         Color::Rgb(r, ..) => r,
//         _ => 0,
//     };
//     println!("{:?}", res);
//
//     let num = Some(4);
//     match num {
//         Some(x) if x % 2 == 0 => println!("The number {x} is even"),
//         Some(x) => println!("The number {x} is odd"),
//         None => (),
//     }
//
//     let x = 4;
//     let y = true;
//     match x {
//         4 | 5 | 6 if y => println!("yes"),
//         _ => println!("no"),
//     }
//     println!("==============================");
//
//     enum Message {
//         Hello { id: i32 },
//     }
//     let msg = Message::Hello { id: 0 };
//     match msg {
//         Message::Hello {
//             id: id_variable @ 3..=7,
//         } => println!("Found an id in range: {id_variable}"),
//         Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
//         Message::Hello { id: id_another } => println!("Found some other id: {id_another}"),
//     }
// }

// static HELLO_WORLD: &str = "Hello, world!";
//
// fn main() {
//     let mut num = 5;
//     let address = 0x012345usize;
//     let r = address as *mut i32;
//
//     let r1 = &num as *const i32;
//     let r2 = &mut num as *mut i32;
//
//     unsafe {
//         println!("num address: {:p}", &num);
//         println!("r1 is: {}", *r1);
//         // println!("r2 is: {}", *r2);
//     }
//
//     unsafe fn dangerous(r: *mut i32) {
//         println!("r2 is: {}", *r);
//     }
//
//     unsafe {
//         dangerous(r2);
//     }
//
//     let mut vector = vec![1, 2, 3, 4, 5, 6];
//     let (left, right) = split_at_mut(&mut vector, 3);
//     println!("{:?}", left);
//     println!("{:?}", right);
//
//     extern "C" {
//         fn abs(input: i32) -> i32;
//     }
//
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
//
//     println!("{}", HELLO_WORLD);
//
//     add_to_count(42);
//     unsafe { println!("{}", COUNTER); }
//
//     println!("===============================");
//
//     let person = Human;
//     Pilot::fly(&person);
//     Wizard::fly(&person);
//     person.fly();
//
//     println!("===============================");
//
//     println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
//
//     println!("===============================");
//
//     let p = Point { x: 1, y: 3 };
//     p.outline_print();
//
//     println!("===============================");
//
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {w}");
//
//     println!("===============================");
//
//     println!("do_twice: {}", do_twice(add_one, 42));
// }
//
// fn add_one(x: i32) -> i32 {
//     x + 1
// }
//
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }
//
// static mut COUNTER: u32 = 0;
//
// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }
//
// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//
//     assert!(mid <= len);
//
//     // (&mut values[..mid], &mut values[mid..])
//     let mut ptr = values.as_mut_ptr();
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }


// fn main() {
//     let my_vec = my_vec!(1,2,3);
//     println!("{:?}", my_vec);
//     println!("========================");
//
//     // let a = Some(1);
//     // a.take();
//     let a = [1, 2, 3];
//     let b = &a[1..2];
//
//     // let mut c: i32 = 0;
//     // fn foo() -> ! {
//     //     ()
//     // }
//     // c = foo();
//
//     // fn bar() -> Result<(), ()> {
//     //     Ok(())
//     // }
//     // let d = bar().unwrap();
//
//     let a = Foo { foo: 42, bar: 1 };
//     let b = Foo { foo: 41, bar: 0 };
//     println!("a > b : {}", a > b);
//
//     let mut c = Vec::new();
//     c.push(&a);
//     c.push(&b);
//     println!("vec before sort: {:?}", c);
//
//     c.sort();
//     println!("vec after sort: {:?}", c);
//
//     let r = c.pop();
//     match r {
//         None => { println!("nothing to see") }
//         Some(s) => { println!("last el in array: {:?}", *s) }
//     }
//
//     let mut d = a.clone();
//     println!("after clone: {:?}", d);
//     Foo::add_foo(&mut d); //equals d.add_foo();
//     println!("after mutating: {:?}", d);
//
//     let e = d;
//     println!("after copy e == d : {}", e == d);
//
//     let mut hasher = DefaultHasher::new();
//     d.hash(&mut hasher);
//     println!("hash: {}", hasher.finish());
//
//     let mut new_hasher = DefaultHasher::new();
//     e.hash(&mut new_hasher);
//     println!("hash: {}", new_hasher.finish());
//
//     let f = Foo {
//         foo: 42,
//         ..Default::default()
//     };
//     println!("f with default: {:?}", f);
//
//     // let g = Box::new(f);
//     // let j = g;
//     // println!("{:?} {:?}", g, j);
// }


// fn main() {
//     let source = RwLock::new(0);
//     // RefCell::new(0); // can not shared between threads !Sync
//     // MutexCell::new(0); // ok
//     // Cell::new(0); // can not shared between threads !Sync
//     let source_to_arc = Arc::new(source);
//
//     let timer = Instant::now();
//     let mut handlers = vec![];
//     for _ in 0..10 {
//         let str_to_arc_ref = Arc::clone(&source_to_arc);
//         let handler = foo_read(str_to_arc_ref);
//         handlers.push(handler);
//     }
//
//     let str_to_arc_ref = Arc::clone(&source_to_arc);
//     let handler = foo_write(str_to_arc_ref);
//     handlers.push(handler);
//
//     for handler in handlers {
//         handler.join().unwrap();
//     }
//
//     let duration = timer.elapsed();
//     println!("Result: {}, time passed: {:?}", source_to_arc.read().unwrap(), duration);
// }
//
// fn foo_read(source: Arc<RwLock<i32>>) -> std::thread::JoinHandle<()> {
//     let handler =
//         std::thread::spawn(move || {
//             std::thread::sleep(Duration::from_secs(2));
//             if let Ok(result) = source.try_read() {
//                 println!("Strong count: {} in {:?} inside foo.", Arc::strong_count(&source), result);
//             }
//         });
//
//     handler
// }
//
// fn foo_write(source: Arc<RwLock<i32>>) -> std::thread::JoinHandle<()> {
//     let handler =
//         std::thread::spawn(move || {
//             std::thread::sleep(Duration::from_secs(2));
//             if let Ok(mut lock_result) = source.write() {
//                 *lock_result = thread_rng().gen_range(0..100);
//             }
//             println!("Strong count: {} in {:?} inside foo.", Arc::strong_count(&source), source.try_read().unwrap());
//         });
//
//     handler
// }
//
// fn foo(source: Arc<MutexCell>) {
//     std::thread::sleep(Duration::from_secs(2));
//     source.set(thread_rng().gen_range(0..100));
//     println!("Strong count: {} in {:?} inside foo.", Arc::strong_count(&source), source.get());
// }
//
// #[derive(Debug)]
// struct MutexCell {
//     field: Mutex<Cell<i32>>,
// }
//
// impl MutexCell {
//     fn new(i: i32) -> MutexCell {
//         MutexCell {
//             field: Mutex::new(Cell::new(i)),
//         }
//     }
//
//     fn get(&self) -> i32 {
//         self.field.lock().unwrap().get()
//     }
//
//     fn set(&self, i: i32) {
//         self.field.lock().unwrap().set(i);
//     }
// }
//
//
// fn some_work() {
//     std::thread::sleep(Duration::from_secs(2));
//     println!("Just sleep in... {:?}", std::thread::current());
// }
//
// fn main() {
//     let source = Arc::new(MutexCell::new(0));
//
//     let thread_pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(3)
//         .build()
//         .unwrap();
//
//     let timer = Instant::now();
//     thread_pool.scope(|x| {
//         for _ in 0..9 {
//             let source_ref = Arc::clone(&source);
//             x.spawn(|_| {
//                 // some_work();
//                 foo(source_ref);
//             });
//         }
//     });
//     let duration = timer.elapsed();
//     println!("Result value: {}, time passed: {:?}", source.get(), duration);
// }

// #[tokio::main]
// async fn main() {
//     let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Can not create listener!");
//
//     loop {
//         if let Ok((mut socket, address)) = listener.accept().await {
//             println!("new client address: {}", address);
//             tokio::spawn(async move {
//                 let mut buffer = [0; 1024];
//
//                 loop {
//                     let n = match socket.read(&mut buffer).await {
//                         Ok(n) => n,
//                         Err(e) => {
//                             println!("Error occurred by reading socket: {} !", e);
//                             return;
//                         }
//                     };
//
//                     if n > 0 {
//                         let data = String::from_utf8_lossy(&buffer[0..n]);
//                         println!("Read something: {:?}", data);//&buffer[0..n]);
//                     }
//
//                     if let Err(e) = socket.write_all(&buffer[0..n]).await {
//                         println!("Error occurred by writing socket: {} !", e);
//                         return;
//                     }
//                 }
//             });
//         }
//     }
// }
//

// fn main() {
//     let (tx, rx) = channel();
//
//     let tx_cloned = tx.clone();
//
//     std::thread::spawn(move || {
//         let arr = ["Hello ", "from ", "thread_id: ", &*thread_id::get().to_string()]
//             .iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<String>>();
//
//         for elem in arr {
//             tx.send(elem).unwrap();
//             println!("Send message!");
//             std::thread::sleep(Duration::from_secs(3));
//         }
//     });
//
//     std::thread::spawn(move || {
//         let arr = ["Hello ", "from ", "thread_id: ", &*thread_id::get().to_string()]
//             .iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<String>>();
//
//         for elem in arr {
//             tx_cloned.send(elem).unwrap();
//             println!("Send message!");
//             std::thread::sleep(Duration::from_secs(2));
//         }
//     });
//
//     for received_data in rx {
//         println!("Received data: {}", received_data);
//     }
// }

// fn main() {
//     let source = AtomicU8::new(0);
//
//     let thread_pool = rayon::ThreadPoolBuilder::new()
//         .num_threads(3)
//         .build()
//         .unwrap();
//
//     thread_pool.scope(|x| {
//         for _ in 0..10 {
//             let source_ref = &source;
//             x.spawn(|_| {
//                 let value = thread_rng().gen_range(0..=100);
//                 source_ref.store(value, SeqCst);
//                 println!("From thread: {}, value is: {}", thread_id::get(), source_ref.load(SeqCst));
//             });
//         }
//     });
//
//     println!("Result: {}", source.load(SeqCst));
// }

#[derive(Default, Debug, Serialize, Deserialize)]
struct Rubles(u32);

#[derive(Default, Debug, Serialize, Deserialize)]
struct Kopecks(u8);

#[derive(Default, Debug, Serialize, Deserialize)]
struct Pay {
    rubles: Rubles,
    kopecks: Kopecks,
}

impl Pay {
    fn new(r: u32, k: u8) -> Pay {
        if k < 100 {
            Pay {
                rubles: Rubles(r),
                kopecks: Kopecks(k),
            }
        } else {
            let k_new: u8 = k % 100;
            let adding_r: u32 = (k / 100) as u32;
            Pay {
                rubles: Rubles(r + adding_r),
                kopecks: Kopecks(k_new),
            }
        }
    }
}

#[derive(Debug)]
enum List<'a, T> {
    Cons(T, &'a List<'a, T>),
    Nil,
}

struct DefaultUser {
    name: String,
}

trait SomethingCool {
    type Output;
    fn cooling(&self) -> Self::Output;
}
// impl SomethingCool for DefaultUser {
//     type Output = String;
//
//     fn cooling(&self) -> Self::Output {
//         let a = &self.name;
//         return *a;
//     }
// }

fn main() {
    let path = "serde.txt";
    let mut file = File::create(path).expect("file open error!");
    let data = serde_json::to_string(&Pay::new(100, 255)).expect("error to serialize!");
    file.write_all(data.as_bytes()).expect("error to write in file!");

    let list = List::Cons(1, &List::Cons(2, &List::Nil));
    println!("{:?}", list);


    let mut test = vec![1, 2, 3];
    for i in test.iter(){}
    for i in test.iter_mut(){}
    for i in test.into_iter() {}// consume vec! ownership on all values
}