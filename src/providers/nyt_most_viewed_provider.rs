use crate::panes::{Link, LinkListPane, Pane};
use async_trait::async_trait;
use serde::Deserialize;
use std::error::Error;

use super::ContentProvider;

pub struct NytWireProvider {
    api_key: String,
    limit: usize,
}

impl NytWireProvider {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            limit: 5,
        }
    }

    async fn query_nyt(&self) -> Result<NytResultBundle, Box<dyn Error>> {
        let uri = format!(
            "https://api.nytimes.com/svc/mostpopular/v2/viewed/1.json?api-key={}",
            self.api_key
        );

        let result_bundle = reqwest::get(uri).await?.json::<NytResultBundle>().await?;

        Ok(result_bundle)
    }
}

#[async_trait]
impl ContentProvider for NytWireProvider {
    async fn build_pane(&self) -> Box<dyn Pane> {
        let nyt_links = self.query_nyt().await.unwrap();

        let links = nyt_links
            .results
            .into_iter()
            .map(|r| Link {
                title: r.title,
                url: r.url,
                subheading: r.r#abstract,
            })
            .take(self.limit)
            .collect();

        let pane = LinkListPane::new(links);

        Box::new(pane)
    }

    async fn download_content(&self) -> Option<String> {
        Some("downloaded article".to_owned())
    }
}

#[derive(Deserialize, Debug)]
pub struct NytStory {
    section: String,
    subsection: String,
    title: String,
    r#abstract: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct NytResultBundle {
    num_results: usize,
    results: Vec<NytStory>,
    status: String,
}
