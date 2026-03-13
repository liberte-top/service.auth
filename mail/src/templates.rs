use minijinja::{context, Environment};
use std::collections::HashMap;

use crate::proto::liberte::mail::v1::Recipient;

pub struct RenderedEmail {
    pub subject: String,
    pub text_body: String,
    pub html_body: String,
}

pub struct TemplateRenderer {
    env: Environment<'static>,
}

impl TemplateRenderer {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.add_template(
            "auth.verify_email.subject",
            include_str!("../templates/auth/verify_email/subject.txt"),
        )
        .expect("verify subject template");
        env.add_template(
            "auth.verify_email.text",
            include_str!("../templates/auth/verify_email/body.txt"),
        )
        .expect("verify text template");
        env.add_template(
            "auth.verify_email.html",
            include_str!("../templates/auth/verify_email/body.html.j2"),
        )
        .expect("verify html template");

        env.add_template(
            "auth.login_link.subject",
            include_str!("../templates/auth/login_link/subject.txt"),
        )
        .expect("login subject template");
        env.add_template(
            "auth.login_link.text",
            include_str!("../templates/auth/login_link/body.txt"),
        )
        .expect("login text template");
        env.add_template(
            "auth.login_link.html",
            include_str!("../templates/auth/login_link/body.html.j2"),
        )
        .expect("login html template");

        Self { env }
    }

    pub fn render(
        &self,
        template_id: &str,
        recipient: &Recipient,
        variables: &HashMap<String, String>,
    ) -> Result<RenderedEmail, String> {
        let common = context! {
            recipient_email => recipient.email.clone(),
            recipient_name => recipient.display_name.clone(),
            action_url => variables.get("action_url").cloned().unwrap_or_default(),
            raw_token => variables.get("raw_token").cloned().unwrap_or_default(),
            destination_sentence => variables.get("destination_sentence").cloned().unwrap_or_default(),
            expires_in => variables.get("expires_in").cloned().unwrap_or_default(),
        };

        let subject = self
            .env
            .get_template(&format!("{template_id}.subject"))
            .map_err(|error| error.to_string())?
            .render(common.clone())
            .map_err(|error| error.to_string())?;
        let text_body = self
            .env
            .get_template(&format!("{template_id}.text"))
            .map_err(|error| error.to_string())?
            .render(common.clone())
            .map_err(|error| error.to_string())?;
        let html_body = self
            .env
            .get_template(&format!("{template_id}.html"))
            .map_err(|error| error.to_string())?
            .render(common)
            .map_err(|error| error.to_string())?;

        Ok(RenderedEmail {
            subject,
            text_body,
            html_body,
        })
    }
}
