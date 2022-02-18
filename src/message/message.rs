use std::fmt::{Display, Formatter,Result};

/// Used to build the message content.
/// ## Examples
/// ```
/// use smail::message::Message;
///
/// fn main(){
///     let mut message = Message::new();
///     message.from(String::from("Rust"));
///     message.to(String::from("Ms.BlueStar"));
///     message.subject(String::from("Happy NewYear"));
///     message.content(String::from("Test smtp send email!!!"));
/// }
/// ```
pub struct Message{
    /// Sender email address
    pub sender:String,
    /// Receiver email address
    pub receiver:String,
    /// Email subject
    pub subject:String,
    /// Email Body
    pub content:String
}

impl Message {
    /// Generates an empty message
    pub fn new()->Message{
        Message{
            sender:String::new(),
            receiver:String::new(),
            subject:String::new(),
            content:String::new(),
        }
    }
    /// Configure the sender information
    pub fn from(&mut self,sender:String){
        self.sender = sender;
    }
    /// Configure the receiver information
    pub fn to(&mut self,receiver:String){
        self.receiver = receiver;
    }
    /// Configure the email subject
    pub fn subject(&mut self,subject:String){
        self.subject = subject;
    }
    /// Configure the email content
    pub fn content(&mut self,content:String){
        self.content = content;
    }
}

impl Display for Message{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f,"from:{}\r\nto:{}\r\nsubject:{}\r\n\r\n{}\r\n.\r\n",self.sender,self.receiver,self.subject,self.content)
    }
}