use async_trait::async_trait;

use super::{ContentDownloader, Pane};

#[derive(Debug)]
pub struct LinkListPane {
    links: Vec<Link>,
}

impl LinkListPane {
    pub fn new(links: Vec<Link>) -> Self {
        Self { links }
    }

    pub fn links(&self) -> &[Link] {
        &self.links
    }
}

impl Pane for LinkListPane {
    fn render_html(&self) -> String {
        let mut r = String::new();

        for link in &self.links {
            r.extend(link.render_html().drain(..));
        }

        r.push_str("<a href='/download.mobi' download>download</a>");

        r
    }

    fn download_path(&self) -> Option<String> {
        self.links.first().and_then(|l| Some(l.url.clone()))
    }
}

#[derive(Debug)]
pub struct Link {
    pub title: String,
    pub url: String,
    pub subheading: String,
}

impl Link {
    fn render_html(&self) -> String {
        let title = &self.title;
        let url = &self.url;

        format!("<a href='{url}' style='white-space: nowrap; overflow: hidden; text-overflow: ellipsis; display: block;'>{title}</a>\n")
    }
}

struct LinkDownloader {
    url: String,
}

#[async_trait]
impl ContentDownloader for LinkDownloader {
    async fn download_content(&self) -> String {
        let response = reqwest::get(&self.url).await;
        let response = response.unwrap();

        response.text().await.unwrap()
    }
}
