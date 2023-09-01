use async_trait::async_trait;
use lettre::{
    address,
    message::header::ContentType,
    transport::smtp::{self, authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};
use scraper::{Html, Selector};
use std::thread::sleep;
use std::time::Duration;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("Email sending error: {0}")]
    EmailSending(#[from] lettre::error::Error),

    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(#[from] address::AddressError),

    #[error("SMTP server error: {0}")]
    SmtpServer(#[from] smtp::Error),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("HTML parsing error: {0}")]
    HtmlParsing(#[from] scraper::error::SelectorErrorKind<'static>),
}

// EmailSender trait for sending emails
#[async_trait]
trait EmailSender {
    async fn send_email(
        &self,
        from_email: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<Response, AppError>;
}

// SmtpEmailSender implements the EmailSender trait for SmtpTransport
struct SmtpEmailSender {
    transport: SmtpTransport,
}

#[async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send_email(
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

        Ok(self.transport.send(&email)?)
    }
}

// WebScraper trait for fetching HTML
#[async_trait]
trait WebScraper {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError>;
}

// ReqwestWebScraper implements the WebScraper trait using reqwest
struct ReqwestWebScraper;

#[async_trait]
impl WebScraper for ReqwestWebScraper {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError> {
        let response = reqwest::get(url).await?;
        let html = response.text().await?;
        Ok(html)
    }
}

// ChristmasChecker encapsulates the checking logic
struct ChristmasChecker<S: EmailSender, W: WebScraper> {
    email_sender: S,
    web_scraper: W,
    url: String,
}

impl<S, W> ChristmasChecker<S, W>
where
    S: EmailSender,
    W: WebScraper,
{
    async fn check_and_send_email(&self, from_email: &str, to_email: &str) -> Result<(), AppError> {
        let answer_selector = Selector::parse("a#answer")?;

        loop {
            let html = self.web_scraper.fetch_html(&self.url).await?;

            let document = Html::parse_document(&html);

            if let Some(answer_node) = document.select(&answer_selector).next() {
                let answer = answer_node.value().attr("title").unwrap_or_default();

                if answer == "YES" {
                    self.email_sender
                        .send_email(
                            from_email,
                            to_email,
                            "It's Christmas!",
                            "According to http://itischristmas.com, it is Christmas!",
                        )
                        .await?;
                    break;
                }
            }

            sleep(Duration::from_secs(3600));
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let smtp_server = "smtp.gmail.com";
    let smtp_port = 587;
    let smtp_username = "your_smtp_username";
    let smtp_password = "your_smtp_password";

    let from_email = "from_email@example.com";
    let to_email = "to_email@example.com";
    let url = "https://isitchristmas.com/";

    let email_transport = SmtpTransport::relay(smtp_server)?
        .port(smtp_port)
        .credentials(Credentials::new(
            smtp_username.to_owned(),
            smtp_password.to_owned(),
        ))
        .build();

    let web_scraper = ReqwestWebScraper;

    let christmas_checker = ChristmasChecker {
        email_sender: SmtpEmailSender {
            transport: email_transport,
        },
        web_scraper,
        url: url.to_owned(),
    };

    christmas_checker
        .check_and_send_email(from_email, to_email)
        .await?;

    Ok(())
}
