use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::{Message, SmtpTransport, Transport};
use scraper::{Html, Selector};
use std::thread::sleep;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Email error: {0}")]
    Letter(#[from] lettre::error::Error),

    #[error("Address error: {0}")]
    Address(#[from] lettre::address::AddressError),

    #[error("SMTP error: {0}")]
    Smtp(#[from] lettre::transport::smtp::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Selector error: {0}")]
    Selector(#[from] scraper::error::SelectorErrorKind<'static>),
}

// Define a trait for email sending
trait EmailSender {
    fn send_email(
        &self,
        from_email: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<Response, AppError>;
}

// Implement the EmailSender trait for SmtpTransport
impl EmailSender for SmtpTransport {
    fn send_email(
        &self,
        from_email: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<Response, AppError> {
        let email = Message::builder()
            .from(from_email.parse()?)
            .to(to_email.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_owned())?;

        Ok(self.send(&email)?)
    }
}

#[async_trait]
trait WebScraper {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError>;
}

// Implement the WebScraper trait using reqwest
struct ReqwestWebScraper;

#[async_trait]
impl WebScraper for ReqwestWebScraper {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError> {
        let response = reqwest::get(url).await?;
        let html = response.text().await?;
        Ok(html)
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // URL of the website to check
    let url = "https://isitchristmas.com/";

    // SMTP credentials
    let smtp_username = "your_smtp_username";
    let smtp_password = "your_smtp_password";

    // Email configuration
    let from_email = "from_email@example.com";
    let to_email = "to_email@example.com";
    let email_subject = "It's Christmas!";
    let email_body = "According to http://itischristmas.com, it is Christmas!";

    // SMTP server configuration (e.g., for Gmail)
    let smtp_server = "smtp.gmail.com";
    let smtp_port = 587;

    let email_transport = SmtpTransport::relay(smtp_server)?
        .port(smtp_port)
        .credentials(Credentials::new(
            smtp_username.to_owned(),
            smtp_password.to_owned(),
        ))
        .build();

    let web_scraper = ReqwestWebScraper;
    let answer_selector = Selector::parse("a#answer")?;

    // Check if it's Christmas and send email
    loop {
        let html = web_scraper.fetch_html(url).await?;

        // Parse the HTML using the scraper crate
        let document = Html::parse_document(&html);

        if let Some(answer_node) = document.select(&answer_selector).next() {
            let answer = answer_node.value().attr("title").unwrap_or_default();

            if answer == "YES" {
                email_transport.send_email(from_email, to_email, email_subject, email_body)?;
                break;
            }
        }

        // Sleep for an hour before checking again
        sleep(Duration::from_secs(3600));
    }

    Ok(())
}
