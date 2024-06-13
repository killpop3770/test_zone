use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::ops::Deref;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

pub struct MyBox<T: Debug + Display> {
    pub refer: T,
}

impl<T: Debug + Display> MyBox<T> {
    pub fn new(refer: T) -> MyBox<T> {
        MyBox {
            refer
        }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.refer))
    }
}

impl<T: Debug + Display> Debug for MyBox<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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