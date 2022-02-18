///
/// # Direct use Examples
///

use smail::authentication::Certificate;
use smail::message::Message;
use smail::transmission::SmtpService;

fn main() {
    //1. Create authentication information.
    let certificate = Certificate::new("usename@qq.com".to_string(), "password".to_string());
    //2. Connect to the SMTP server and authenticate it.
    let smtp_transport = SmtpService::address(String::from("smtp.qq.com"), 587)
        .verify(certificate)
        .connect();
    //3. Configure the sender email address and recipient email address.
    let mut channel = smtp_transport.build_channel(
        String::from("from_username@qq.com"),
        vec![String::from("to_usename@qq.com")],
    );
    //4. Build content based on the email format.
    let mut message = Message::new();
    message.from(String::from("from_name"));
    message.to(String::from("to_name"));
    message.subject(String::from("Email Subject"));
    message.content(String::from("Hello SMAIL"));
    //5. Send emails.
    channel.send_email(message);
    //6. Close the connection.
    channel.close();
}
