use crate::base64;

/// Used to set up SMTP authentication information
/// ## Examples
/// ```
/// use smail::authentication::Certificate;
///
/// fn main() {
///     let certificate = Certificate::new("820813748@qq.com".to_string(), "utszlhtihttebbie".to_string());
/// }
/// ```
pub struct Certificate{
    username:String,
    password:String
}

impl Certificate{
    pub fn new(username:String,password:String)->Certificate{
        Certificate{
            username,
            password
        }
    }
    pub fn get_base64(self)->(String,String){
        let mut username = base64::encode(self.username);
        username.push_str("\r\n");
        let mut password = base64::encode(self.password);
        password.push_str("\r\n");
        (username,password)
    }
}