//! WARNING: there are some hard-coded values in here which you'll want to
//! modify! In particular
//!
//! - `To:` and `Reply-To:` fields in the email header are hard-coded!
//! - smtp host is hard-coded (`smtp.gmail.com`)
//!
//! [send_email] will return an error until you open up this file and customize
//! this!

#[allow(unused_imports)]
use anyhow::{Error, Result};
#[allow(unused_imports)]
#[cfg(feature = "enable_smtp_email")]
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
#[allow(unused_imports)]
#[cfg(feature = "enable_smtp_email")]
use std::env;

#[allow(unused_variables)]
#[cfg(feature = "enable_smtp_email")]
pub async fn send_email(to: &str, subject: &str, msg: &str) -> Result<()> {
    Err(Error::msg(
        "hard-coded values in send_email must be adjusted",
    ))
    // let email = Message::builder()
    //     .from("yoursite.example.com <noreply@example.com>".parse()?)
    //     .reply_to("yoursite.example.com <noreply@example.com>".parse()?)
    //     .to(to.parse()?)
    //     .subject(subject)
    //     .header(ContentType::TEXT_PLAIN)
    //     .body(String::from(msg))?;

    // let username = env::var("SMTP_EMAIL_USERNAME")?;
    // let password = env::var("SMTP_EMAIL_PASSWORD")?;

    // let creds = Credentials::new(username, password);

    // let mailer =
    // AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(creds)
    //     .build();

    // mailer.send(email).await?;
    // Ok(())
}

#[cfg(not(feature = "enable_smtp_email"))]
pub async fn send_email(to: &str, subject: &str, msg: &str) -> Result<()> {
    println!("Would send email:\n\tTo: {to}\n\tSubject: {subject}\n\tBody:\n{msg}\n===\n");

    Ok(())
}
