use async_trait::async_trait;
use opentelemetry::trace::TraceContextExt;
use opentelemetry::{global, propagation::Injector};
use std::sync::Arc;
use tonic::transport::Channel;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::{
    mail_proto::liberte::mail::v1::mail_service_client::MailServiceClient,
    service::config::ConfigService, telemetry::TRACE_ID_HEADER,
};

use crate::mail_proto::liberte::mail::v1::{KeyValue, Recipient, SendTemplateEmailRequest};

struct MetadataInjector<'a>(&'a mut tonic::metadata::MetadataMap);

impl Injector for MetadataInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        if let (Ok(key), Ok(value)) = (
            tonic::metadata::MetadataKey::from_bytes(key.as_bytes()),
            value.parse::<tonic::metadata::MetadataValue<_>>(),
        ) {
            self.0.insert(key, value);
        }
    }
}

#[async_trait]
pub trait MailClientService: Send + Sync {
    async fn send_template_email(
        &self,
        template_id: &str,
        locale: &str,
        to_email: &str,
        display_name: Option<&str>,
        variables: Vec<(String, String)>,
        metadata: Vec<(String, String)>,
    ) -> Result<(), String>;
}

pub struct GrpcMailClientService {
    config: Arc<dyn ConfigService>,
}

impl GrpcMailClientService {
    pub fn new(config: Arc<dyn ConfigService>) -> Self {
        Self { config }
    }

    async fn connect(&self) -> Result<MailServiceClient<Channel>, String> {
        let addr = self
            .config
            .mail_grpc_addr()
            .ok_or_else(|| "mail grpc address not configured".to_owned())?;
        MailServiceClient::connect(addr.to_owned())
            .await
            .map_err(|error| error.to_string())
    }
}

#[async_trait]
impl MailClientService for GrpcMailClientService {
    async fn send_template_email(
        &self,
        template_id: &str,
        locale: &str,
        to_email: &str,
        display_name: Option<&str>,
        variables: Vec<(String, String)>,
        metadata: Vec<(String, String)>,
    ) -> Result<(), String> {
        let mut client = self.connect().await?;
        let trace_id = tracing::Span::current()
            .context()
            .span()
            .span_context()
            .trace_id()
            .to_string();
        let mut request = tonic::Request::new(SendTemplateEmailRequest {
            template_id: template_id.to_owned(),
            locale: locale.to_owned(),
            recipient: Some(Recipient {
                email: to_email.to_owned(),
                display_name: display_name.unwrap_or_default().to_owned(),
            }),
            variables: variables
                .into_iter()
                .map(|(key, value)| KeyValue { key, value })
                .collect(),
            metadata: metadata
                .into_iter()
                .map(|(key, value)| KeyValue { key, value })
                .collect(),
            idempotency_key: trace_id.clone(),
        });

        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(
                &tracing::Span::current().context(),
                &mut MetadataInjector(request.metadata_mut()),
            )
        });

        if let Ok(value) = trace_id.parse() {
            request.metadata_mut().insert(TRACE_ID_HEADER, value);
        }

        client
            .send_template_email(request)
            .await
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}
