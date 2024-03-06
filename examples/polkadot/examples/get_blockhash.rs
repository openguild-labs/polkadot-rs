use rpc_provider::{defaults::WS_URL, jsonrpsee::JsonrpseeClient, rpc_params, Request};
use sp_core::H256;
use types_support::metadata::v15::polkadot_rpc::PolkadotRpcMethod;

#[tokio::main]
async fn main() {
	let client = JsonrpseeClient::new(WS_URL).await.unwrap();
	let method = PolkadotRpcMethod::ChainGetBlockHash.as_string();
	let output = client.request::<H256>(&method, rpc_params!(Some(0))).await.unwrap();

	println!("Blockhas: {output:?}");
}
