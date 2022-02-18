use std::fmt::{Display, Formatter, Result};
pub trait Command {}

///EHLO 命令
pub struct Ehlo {
    smtp_address: String,
}

impl Command for Ehlo {}

impl Display for Ehlo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "EHLO {}\r\n", self.smtp_address)
    }
}

impl Ehlo {
    pub fn new(smtp_address: String) -> Ehlo {
        Ehlo { smtp_address }
    }
}

pub struct AuthLogin {}

impl Command for AuthLogin {}

impl Display for AuthLogin {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "AUTH LOGIN\r\n")
    }
}

///MAIL FROM 命令
pub struct MailFrom {
    mail_from: String,
}

impl Command for MailFrom {}

impl Display for MailFrom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "MAIL FROM:<{}>\r\n", self.mail_from)
    }
}

impl MailFrom {
    pub fn new(mail_from: String) -> MailFrom {
        MailFrom { mail_from }
    }
}

///RCPT TO 命令
pub struct RcptTo {
    rcpt_to: String,
}

impl Command for RcptTo {}

impl Display for RcptTo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "RCPT TO:<{}>\r\n", self.rcpt_to)
    }
}

impl RcptTo {
    pub fn new(rcpt_to: String) -> RcptTo {
        RcptTo { rcpt_to }
    }
}

///Data 命令
pub struct Data;

impl Command for Data {}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("DATA\r\n")
    }
}

///QUIT 命令
pub struct Quit;

impl Command for Quit {}

impl Display for Quit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str("QUIT\r\n")
    }
}


