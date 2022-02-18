use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;
use crate::transmission::response::ResponseHandler;
use super::smtp::command::Command;

pub struct SmtpStream {
    stream: TcpStream,
}

impl SmtpStream {
    pub fn new(stream: TcpStream) -> SmtpStream {
        SmtpStream {
            stream
        }
    }
    pub fn send_command<T: Command + Display>(&mut self, command: T) {
        println!("{}",format_args!("{}", command));
        self.stream.write_fmt(format_args!("{}", command)).unwrap();
        ResponseHandler::processing(&mut self.stream);
    }
    pub fn send_data(&mut self, data: String) {
        println!("{}", data);
        self.stream.write(data.as_bytes()).unwrap();
        ResponseHandler::processing(&mut self.stream);
    }
}

