use std::{net::SocketAddr, sync::Arc};

use tonic::transport::Server;
use tonic_health::server::health_reporter;

mod config;
mod proto;
mod provider;
mod server;
mod templates;

use config::Config;
use proto::liberte::mail::v1::mail_service_server::MailServiceServer;
use provider::ResendProvider;
use server::MailGrpcServer;
use templates::TemplateRenderer;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::from_env());
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let renderer = Arc::new(TemplateRenderer::new());
    let provider = Arc::new(ResendProvider::new(config.clone()));
    let service = MailGrpcServer::new(renderer, provider);
    let (mut reporter, health_service) = health_reporter();
    reporter
        .set_serving::<MailServiceServer<MailGrpcServer>>()
        .await;

    eprintln!("starting service.mail gRPC on {}", addr);
    Server::builder()
        .add_service(health_service)
        .add_service(MailServiceServer::new(service))
        .serve(addr)
        .await
        .expect("mail gRPC serve error");
}
