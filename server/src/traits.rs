use async_trait::async_trait;
pub mod author;
pub mod post;
pub mod server;

#[async_trait]
pub trait GRPCService {
    async fn start(self) -> Result<(), Box<dyn std::error::Error>>;
}