use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    transport::smtp::authentication::Credentials,
};
use rand::Rng;
use crate::config::environment::Config;

pub struct EmailService {
    config: Config,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailService {
    pub fn new(config: Config) -> Self {
        let creds = Credentials::new(
            config.email_user.clone(),
            config.email_password.clone(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        EmailService { config, mailer }
    }

    pub fn generate_verification_code() -> String {
        let mut rng = rand::thread_rng();
        let code: u32 = rng.gen_range(100000..999999);
        code.to_string()
    }

    pub async fn send_verification_email(&self, to_email: &str, verification_code: &str) -> Result<(), String> {
        let email = Message::builder()
            .from(format!("Image to Text Service <{}>", self.config.email_user).parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Verify Your Email Address")
            .body(format!(
                "Welcome to Image to Text Service!\n\n\
                Please use the following verification code to verify your email address:\n\n\
                {}\n\n\
                This code will expire in 30 minutes.\n\n\
                If you didn't create an account with us, please ignore this email.\n\n\
                Best regards,\n\
                Image to Text Team",
                verification_code
            ))
            .unwrap();

        match self.mailer.send(email).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to send email: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_code_generation() {
        let code = EmailService::generate_verification_code();
        assert_eq!(code.len(), 6);
        assert!(code.parse::<u32>().is_ok());
    }
} 