// local modules
mod server;
mod logs;

use crate::server::bootstrap;

#[tokio::main]
async fn main() {
    bootstrap().await;
}
