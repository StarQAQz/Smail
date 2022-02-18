use std::fmt::{Display, Formatter};
use std::io::{Read};
use std::net::TcpStream;
use std::str;

struct Response {
    code: u32,
    msg: String,
}

impl Response {
    fn parse(info: String) -> Vec<Response> {
        let mut result: Vec<Response> = Vec::new();
        for line in info.lines() {
            let line = line.to_string();
            let (code, msg) = line.split_at(3);
            result.push(Response {
                code: code.to_string().parse().unwrap(),
                msg: msg.trim_matches(|x|x==' '||x=='-').to_string(),
            });
        }
        result
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code:{} msg:{}", self.code, self.msg)
    }
}

pub struct ResponseHandler {}

const BUFFER_SIZE: usize = 1024;

impl ResponseHandler {
    pub fn processing(mut stream: &TcpStream) {
        let mut info:Vec<u8> = Vec::new();
        let mut response_len = 0;
        let mut buffer:[u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        loop{
            match stream.read(&mut buffer){
                Ok(n)=>{
                    response_len+=n;
                    info.extend_from_slice(&mut buffer[..n]);
                    if n<BUFFER_SIZE {
                        break;
                    }
                },
                Err(e)=>panic!("ResponseHandler::processing() Error:[{}]",e)
            }
        }
        let all_response = str::from_utf8(&info[..response_len]).unwrap();
        let all_lines = Response::parse(String::from(all_response));
        all_lines.iter().for_each(|line|println!("{}",line));
        for response in all_lines.into_iter() {
            if response.code < 200 && response.code >= 400 {
                panic!("ResponseHandler::processing() Error:[code:{},msg:{}]", response.code, response.msg);
            }
        }
    }
}