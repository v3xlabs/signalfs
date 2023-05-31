
pub mod http;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    http::boostrap().await;
}
