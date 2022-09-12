use crate::panes::{Link, LinkListPane, Pane};
use async_trait::async_trait;
use serde::Deserialize;
use std::error::Error;

use super::ContentProvider;

pub struct NytMostViewedProvider {
    api_key: String,
    limit: usize,
}

impl NytMostViewedProvider {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.into(),
            limit: 3,
        }
    }

    async fn query_nyt(&self) -> Result<NytResultBundle, Box<dyn Error>> {
        let uri = format!(
            "https://api.nytimes.com/svc/news/v3/content/nyt/all.json?api-key={}&limit={}",
            self.api_key, self.limit
        );

        let result_bundle = reqwest::get(uri).await?.json::<NytResultBundle>().await?;

        Ok(result_bundle)
    }
}

#[async_trait]
impl ContentProvider for NytMostViewedProvider {
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
            .collect();

        let pane = LinkListPane::new(links);

        Box::new(pane)
    }

    async fn download_content(&self) -> Option<String> {
        None
    }
}

#[derive(Deserialize, Debug)]
pub struct NytStory {
    slug_name: String,
    section: String,
    subsection: String,
    title: String,
    r#abstract: String,
    url: String,
    uri: String,
    created_date: String,
    kicker: String,
    subheadline: String,
}

#[derive(Deserialize, Debug)]
pub struct NytResultBundle {
    num_results: usize,
    results: Vec<NytStory>,
    status: String,
}
