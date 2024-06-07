use mpsc::Sender;
use std::{env, fs, io, thread};
use std::ffi::OsStr;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use pulldown_cmark::{Options, Parser};
use crate::structs::ProcessorMessage;


pub fn convert_md_to_html_parallel(files: Vec<PathBuf>,
                                   sender: Sender<ProcessorMessage>) -> Vec<thread::JoinHandle<()>> {
    files.into_iter().enumerate().map(|(index, file)| {
        let new_sender = sender.clone();

        thread::spawn(move || {
            let content = match fs::read_to_string(&file) {
                Ok(content) => content,
                Err(error) => {
                    new_sender
                        .send(ProcessorMessage::Error(
                            format!("Error to read file {:?} : {}", &file, error)
                        )).unwrap();
                    return;
                }
            };

            let parser = Parser::new_ext(&content, Options::empty());
            let mut parsed_data = String::new();
            pulldown_cmark::html::push_html(&mut parsed_data, parser);

            let default_file_name = format!("file{index}");
            let current_file_name = file.file_name().unwrap_or(OsStr::new(&default_file_name)).to_str().unwrap();

            let mut html_file_path = env::current_dir().expect("Can not read directory!");
            html_file_path.push("html");
            html_file_path.push(current_file_name);
            html_file_path.set_extension("html");

            let mut output_file = fs::File::create(html_file_path).expect("Can not create html file!");
            output_file.write_all(parsed_data.as_bytes()).expect("Can not write data in file!");

            new_sender.send(ProcessorMessage::Success(
                format!("File {:?} is processed!", &file)
            )).expect("Failed to send message!");
        })
    }).collect()
}

pub fn converter_celcius_and_fahrenheit() -> Result<f32, &'static str> {
    println!("(1) -> °C to °F | (2) -> °F to °C");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let mode = match user_input.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid input value"),
    };

    match mode {
        1 | 2 => {}
        _ => return Err("Invalid input value"),
    }

    println!("Input your temperature: ");
    let mut temperature: String = String::new();
    io::stdin().read_line(&mut temperature).unwrap();
    let value = match temperature.trim_end().parse::<f32>() {
        Ok(n) => n,
        Err(_) => return Err("Invalid input value"),
    };

    return match mode {
        1 => Ok(value * 9. / 5. + 32.),// (0 °C × 9/5) + 32 = 32 °F
        2 => Ok((value - 32.) * 5. / 9.),// (0 °F − 32) × 5/9 = -17,78 °C
        _ => Err("Unexpected error!"),
    };
}