use crate::panes::Pane;
use async_trait::async_trait;

pub mod nyt_most_viewed_provider;
pub mod nyt_wire_provider;

#[async_trait]
pub trait ContentProvider: Send + Sync {
    async fn build_pane(&self) -> Box<dyn Pane>;
    async fn download_content(&self) -> Option<String>;
}
