use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Foo {
    pub foo: i32,
    pub bar: i32,
}

impl Foo {
    pub fn add_foo(&mut self) {
        self.foo += 1;
    }

    pub fn bad_method(self) {
        println!("something wrong!");
    }
}

impl Default for Foo {
    fn default() -> Self {
        Foo {
            foo: 0,
            bar: 0,
        }
    }
}

impl Hash for Foo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.foo.hash(state);
        self.bar.hash(state);
    }
}

impl Copy for Foo {}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo {
            foo: self.foo.clone(),
            bar: self.bar.clone(),
        }
    }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.foo.cmp(&other.foo)
    }
}

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.foo == other.foo &&
            self.bar == other.bar
    }
}

impl Eq for Foo {}

pub struct Wrapper(pub Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}


pub trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", len);
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub struct EPost {
    content: String,
    state: PostState,
    approve_cnt: usize,
}

impl EPost {
    pub fn new() -> EPost {
        EPost {
            content: String::new(),
            state: PostState::Draft,
            approve_cnt: 0,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn add_text(&mut self, text: &str) {
        match self.state {
            PostState::Draft => self.content.push_str(text),
            _ => (),
        }
    }

    pub fn request_review(&mut self) {
        self.state = match self.state {
            PostState::Draft => PostState::PendingReview,
            PostState::PendingReview => PostState::PendingReview,
            PostState::Published => PostState::Published,
        }
    }

    pub fn reject(&mut self) {
        self.state = match self.state {
            PostState::Draft => PostState::Draft,
            PostState::PendingReview => PostState::Draft,
            PostState::Published => PostState::Published,
        };
        self.approve_cnt = 0;
    }

    pub fn approve(&mut self) {
        self.state = match self.state {
            PostState::Draft => PostState::Draft,
            PostState::PendingReview => {
                return if self.approve_cnt == 2 {
                    self.state = PostState::Published
                } else {
                    self.approve_cnt += 1;
                    self.state = PostState::PendingReview
                };
            }
            PostState::Published => PostState::Published,
        };
    }
}

enum PostState {
    Draft,
    PendingReview,
    Published,
}

pub struct NPost {
    content: String,
}

impl NPost {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> NPost {
        NPost {
            content: self.content,
        }
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approve_cnt: usize,
}

impl Post {
    pub fn new() -> Post {
        println!("Create new Post!");
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approve_cnt: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        println!("add text: {} to post", text);
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("transfer to Pending Review!");
        Box::new(PendingReview {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("do nothing...");
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("do nothing...");
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("do nothing...");
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("reject to Draft!");
        Box::new(Draft {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("transfer to Published!");
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        println!("do nothing...");
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        println!("do nothing...");
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        println!("do nothing...");
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[derive(Debug)]
pub struct Node {
    pub(crate) value: i32,
    pub(crate) parent: RefCell<Weak<Node>>,
    pub(crate) children: RefCell<Vec<Rc<Node>>>,
}

pub struct MyBox<T: Debug + Display> {
    pub refer: T,
}

impl<T: Debug + Display> MyBox<T> {
    pub fn new(refer: T) -> MyBox<T> {
        MyBox { refer }
    }

    pub fn print_field(&self) {
        println!("Ok! : {:?}", self.refer);
    }

    pub fn hello(name: &str) {
        println!("Hello, {name}");
    }
}

impl<T: Debug + Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop object {:?} !", self);
    }
}

impl<T: Display + Debug> Display for MyBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.refer))
    }
}

impl<T: Debug + Display> Debug for MyBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("MyBox {}", self.refer))
    }
}

impl<T: Debug + Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.refer
    }
}

// #[derive(Debug)]
// pub struct Cat<'a> {
//     pub name: String,
//     pub age: usize,
//     pub parent: Option<&'a Cat<'a>>,
// }

#[derive(Debug, Clone)]
pub struct Cat {
    pub name: String,
    pub age: usize,
    pub parent: Option<Rc<RefCell<Cat>>>,
}

pub enum ProcessorMessage {
    Success(String),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub article: String,
    pub author: String,
    pub paragraph: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paragraph {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub id: u32,
}
