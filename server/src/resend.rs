use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};

pub struct ResendClient {
  client: Resend,
}

impl ResendClient {
  pub fn new() -> Self {

    ResendClient {
      client: Resend::default(),
    }
  }

  pub async fn send_email(
    &self,
    customer_support_email: &str,
    subject: &str,
    email_body: &str,
  ) -> Result<()> {
    let from = "evan@updates.betternewtab.com";
    let to = [customer_support_email];

    let email = CreateEmailBaseOptions::new(from, to, subject)
      .with_html(email_body);

    let _email = self.client.emails.send(email).await?;
    println!("{:?}", _email);

    Ok(())
  }
}