use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};

fn main() {
    let email = EmailBuilder::new()
        .from(Mailbox::new("发送者的邮箱地址".to_string()))
        .to(Mailbox::new("接收者邮箱地址".to_string()))
        .subject("Test")
        .body("This is a test email!")
        .build()
        .unwrap();
    let creds = Credentials::new("你的邮箱用户名".to_string(), "你的邮箱密码".to_string());
    let mut mailer = SmtpClient::new_simple("邮箱服务器地址")
        .unwrap()
        .credentials(creds)
        .transport();
    let result = mailer.send(email.into());
    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
    assert!(result.is_ok());
}
