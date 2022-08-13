use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("Hello, world!");
    let elapsed = now.elapsed();

    println!("{:?}", elapsed);
}

use std::net::SocketAddr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::http_server::{HttpServerBuilder, HttpServerHandle, RpcModule};
use jsonrpsee::rpc_params;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"say_hello\"}]=trace".parse()?);
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	let (server_addr, _handle) = run_server().await?;
	let url = format!("http://{}", server_addr);

	let client = HttpClientBuilder::default().build(url)?;
	let params = rpc_params!(1_u64, 2, 3);
	let response: Result<String, _> = client.request("say_hello", params).await;
	tracing::info!("r: {:?}", response);

	Ok(())
}

async fn run_server() -> anyhow::Result<(SocketAddr, HttpServerHandle)> {
	let server = HttpServerBuilder::default().build("127.0.0.1:0".parse::<SocketAddr>()?).await?;
	let mut modulex = RpcModule::new(());
	modulex.register_method("say_hello", |_, _| Ok("lo"))?;

	let addr = server.local_addr()?;
	let server_handle = server.start(modulex)?;
	Ok((addr, server_handle))
}