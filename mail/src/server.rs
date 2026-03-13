use std::{collections::HashMap, sync::Arc};

use tonic::{Request, Response, Status};

use crate::{
    proto::liberte::mail::v1::{
        mail_service_server::MailService, SendTemplateEmailRequest, SendTemplateEmailResponse,
    },
    provider::{ProviderMessage, ResendProvider},
    templates::TemplateRenderer,
};

#[derive(Clone)]
pub struct MailGrpcServer {
    renderer: Arc<TemplateRenderer>,
    provider: Arc<ResendProvider>,
}

impl MailGrpcServer {
    pub fn new(renderer: Arc<TemplateRenderer>, provider: Arc<ResendProvider>) -> Self {
        Self { renderer, provider }
    }
}

#[tonic::async_trait]
impl MailService for MailGrpcServer {
    async fn send_template_email(
        &self,
        request: Request<SendTemplateEmailRequest>,
    ) -> Result<Response<SendTemplateEmailResponse>, Status> {
        let request = request.into_inner();
        let recipient = request
            .recipient
            .ok_or_else(|| Status::invalid_argument("recipient is required"))?;

        if recipient.email.trim().is_empty() {
            return Err(Status::invalid_argument("recipient.email is required"));
        }

        let variables: HashMap<String, String> = request
            .variables
            .into_iter()
            .map(|item| (item.key, item.value))
            .collect();

        let rendered = self
            .renderer
            .render(&request.template_id, &recipient, &variables)
            .map_err(Status::invalid_argument)?;

        let result = self
            .provider
            .send(ProviderMessage {
                to_email: recipient.email,
                to_name: (!recipient.display_name.is_empty()).then_some(recipient.display_name),
                subject: rendered.subject,
                text_body: rendered.text_body,
                html_body: rendered.html_body,
            })
            .await
            .map_err(Status::internal)?;

        Ok(Response::new(SendTemplateEmailResponse {
            message_id: result.message_id,
            provider: result.provider.to_owned(),
            status: "accepted".to_owned(),
        }))
    }
}
