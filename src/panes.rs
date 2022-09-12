mod link_list_pane;

use async_trait::async_trait;
pub use link_list_pane::{Link, LinkListPane};

pub trait Pane {
    fn render_html(&self) -> String;
    fn download_path(&self) -> Option<String>;
}

#[async_trait]
pub trait ContentDownloader {
    async fn download_content(&self) -> String;
}
