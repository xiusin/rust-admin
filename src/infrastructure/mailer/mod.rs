pub mod email_sender;
pub mod template;

pub use email_sender::EmailSender;

use crate::config::APPCOFIG;
use crate::worker::common::{Worker, WorkerOpts};
use crate::worker::AppWorker;
use crate::common::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

pub const DEFAULT_FROM_SENDER: &str = "XXXX科技 <xxxx@163.com>";

static EMAILSENDER: OnceCell<EmailSender> = OnceCell::const_new();

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Email {
    pub from: Option<String>,
    pub to: String,
    pub reply_to: Option<String>,
    pub subject: String,
    pub text: String,
    pub html: String,
}

#[derive(Clone)]
pub struct MailerWorker {}

impl AppWorker<Email> for MailerWorker {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Worker<Email> for MailerWorker {
    fn opts() -> WorkerOpts<Email, Self> {
        WorkerOpts::new().queue("mailer")
    }

    async fn perform(&self, email: Email) -> Result<()> {
        let mailer = EMAILSENDER.get_or_init(email_init).await;
        let _ = mailer.mail(&email).await;
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Args {
    pub from: Option<String>,
    pub to: String,
    pub reply_to: Option<String>,
    pub locals: serde_json::Value,
}

async fn email_init() -> EmailSender {
    let config = APPCOFIG.mailer.clone().unwrap().smtp.unwrap();
    EmailSender::smtp(&config).unwrap()
}

pub async fn mail_template(dir: String, args: Args) -> Result<()> {
    let content = template::Template::new(dir).render(&args.locals)?;
    mail(&Email {
        from: args.from.clone(),
        to: args.to.clone(),
        reply_to: args.reply_to.clone(),
        subject: content.subject,
        text: content.text,
        html: content.html,
    })
    .await
}

async fn mail(email: &Email) -> Result<()> {
    MailerWorker::enqueue_async(email.clone())
        .await
        .map_err(Box::from)?;
    Ok(())
}
