use std::net::TcpStream;
use crate::authentication::Certificate;
use crate::message::Message;
use crate::transmission::response::ResponseHandler;
use crate::transmission::smtp::command::*;
use crate::transmission::stream::SmtpStream;

/// The first step in establishing SMTP service communication
///
/// Before sending emails, configure the required information according to the procedure.
pub struct SmtpService {
    smtp_address: String,
    port: u16,
}

impl SmtpService {
    /// Configure the SMTP service address.
    ///
    /// ## Examples
    /// ```
    /// use smail::transmission::SmtpService;
    ///
    /// fn main(){
    ///     let smtp_transport = SmtpService::address(String::from("smtp.qq.com"), 587);
    /// }
    /// ```
    pub fn address(smtp_address: String, port: u16) -> SmtpService {
        SmtpService { smtp_address, port }
    }
    /// Configure the authentication information.
    ///
    /// ## Examples
    /// ```
    /// use smail::transmission::SmtpService;
    ///
    /// fn main(){
    ///     let smtp_transport = SmtpService::address(String::from("smtp.qq.com"), 587);
    ///     smtp_transport..verify(certificate);
    /// }
    /// ```
    pub fn verify(self, certificate: Certificate) -> Verification {
        Verification {
            smtp_service: self,
            certificate,
        }
    }
}

/// The SMTP service and authentication information are available
pub struct Verification {
    smtp_service: SmtpService,
    certificate: Certificate,
}

impl Verification {
    /// Used to establish connections.
    /// ## Examples
    /// ```
    /// use smail::transmission::SmtpService;
    ///
    /// fn main(){
    ///     let smtp_transport = SmtpService::address(String::from("smtp.qq.com"), 587).verify(certificate);
    ///     smtp_transport.connect();
    /// }
    /// ```
    pub fn connect(self) -> SmtpTransport {
        let smtp_service = [self.smtp_service.smtp_address.clone(), self.smtp_service.port.to_string()].join(":");
        let mut smtp_stream = match TcpStream::connect(&smtp_service) {
            Ok(stream) => {
                ResponseHandler::processing(&stream);
                SmtpStream::new(stream)
            },
            Err(e) => panic!("The SMTP service connection fails! Error:{}", e),
        };
        let ehlo = Ehlo::new(self.smtp_service.smtp_address.clone());
        smtp_stream.send_command(ehlo);
        smtp_stream.send_command(AuthLogin {});
        let (username, password) = self.certificate.get_base64();
        smtp_stream.send_data(username);
        smtp_stream.send_data(password);
        SmtpTransport { stream: smtp_stream }
    }
}

/// SMTP service communication channel.
pub struct SmtpTransport {
    stream: SmtpStream,
}

impl SmtpTransport {
    /// This parameter is used to configure sender and recipient emails and establish mail sending channels.
    /// ## Examples
    /// ```
    /// fn main(){
    ///     let mut channel = smtp_transport.build_channel(String::from("820813748@qq.com"), vec![String::from("bluestarz@qq.com")]);
    /// }
    /// ```
    pub fn build_channel(self, sender: String, receivers: Vec<String>) -> EmailChannel {
        let mut smtp_stream = self.stream;
        smtp_stream.send_command(MailFrom::new(sender));
        for receiver in receivers.into_iter() {
            smtp_stream.send_command(RcptTo::new(receiver));
        }
        EmailChannel { stream: smtp_stream }
    }
}

/// The mail communication channel has been established.
///
/// Reusable sending mail data.
///
/// Close the connection channel manually at the end of use.
/// ## Examples
/// ```
/// fn main(){
///     let mut channel = smtp_transport.build_channel(String::from("820813748@qq.com"), vec![String::from("bluestarz@qq.com")]);
///     channel.send_email(message);
///     channel.close();
/// }
/// ```
pub struct EmailChannel {
    stream: SmtpStream,
}

impl EmailChannel {
    /// Sending email Messages
    pub fn send_email(&mut self, message: Message) {
        let smtp_stream = &mut self.stream;
        smtp_stream.send_command(Data {});
        smtp_stream.send_data(message.to_string());
    }
    /// Disable the mail communication channel.
    pub fn close(mut self) {
        self.stream.send_command(Quit {});
    }
}