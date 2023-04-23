use tonic_web_wasm_client::Client;

pub fn grpc_client(url: impl ToString) -> Client {
    Client::new(url.to_string())
}