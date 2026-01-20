use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};

pub struct ResendClient {
    client: Resend,
}

impl ResendClient {
    pub fn new() -> Self {
        tracing::info!("Initializing Resend client");
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
        tracing::info!("Sending email to: {}", customer_support_email);
        let from = "evan@updates.omega-tab.evanrobertson.dev";
        let to = [customer_support_email];

        let email = CreateEmailBaseOptions::new(from, to, subject).with_html(email_body);

        match self.client.emails.send(email).await {
            Ok(_email) => {
                tracing::info!("Successfully sent email to: {}", customer_support_email);
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to send email: {:?}", e);
                Err(e)
            }
        }
    }
}
