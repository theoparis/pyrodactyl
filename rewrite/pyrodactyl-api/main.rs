#![allow(dead_code)]
use std::{error::Error, sync::Arc};

use axum::{Router, routing::get};
use axum_h3::H3Router;
use clap::Parser;
use h3_util::quinn::H3QuinnAcceptor;
use quinn::rustls::{
	self,
	pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};
use tracing_subscriber::{
	layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

mod entities;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
	/// The address to bind the server to.
	#[arg(
		long,
		short = 'b',
		value_name = "ADDR",
		default_value = "[::1]:8080"
	)]
	pub bind_address: std::net::SocketAddr,
}

pub struct State;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	tracing_subscriber::registry()
		.with(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| {
					format!("{}=trace", env!("CARGO_CRATE_NAME")).into()
				}),
		)
		.with(tracing_subscriber::fmt::layer())
		.init();

	let args = Cli::parse();

	tracing::info!("Starting Pyrodactyl API server on {}", args.bind_address);

	let router = H3Router::new(
		Router::new()
			.with_state(Arc::new(State))
			.route("/", get(|| async { "Welcome to Pyrodactyl API!" })),
	);

	let tls_config = Arc::new(make_rustls_server_config());

	let server_config = quinn::ServerConfig::with_crypto(Arc::new(
		quinn::crypto::rustls::QuicServerConfig::try_from(tls_config).unwrap(),
	));
	let endpoint = quinn::Endpoint::server(server_config, args.bind_address)?;
	router.serve(H3QuinnAcceptor::new(endpoint)).await.unwrap();

	Ok(())
}

pub fn make_rustls_server_config() -> rustls::ServerConfig {
	let (cert, key) = generate_self_signed_cert(&["localhost".to_string()])
		.expect("Failed to generate self-signed certificate");
	let mut tls_config = rustls::ServerConfig::builder_with_provider(
		rustls::crypto::ring::default_provider().into(),
	)
	.with_safe_default_protocol_versions()
	.unwrap()
	.with_no_client_auth()
	.with_single_cert(vec![cert.clone()], PrivateKeyDer::Pkcs8(key.clone_key()))
	.unwrap();
	tls_config.alpn_protocols = vec![b"h3".to_vec()];
	tls_config.max_early_data_size = u32::MAX;
	tls_config
}

fn generate_self_signed_cert(
	subject_names: &[String],
) -> Result<
	(CertificateDer<'static>, PrivatePkcs8KeyDer<'static>),
	Box<dyn Error>,
> {
	let cert = rcgen::generate_simple_self_signed(subject_names)?;
	let cert_der = CertificateDer::from(cert.cert);
	let key = PrivatePkcs8KeyDer::from(cert.signing_key.serialize_der());
	Ok((cert_der, key))
}
