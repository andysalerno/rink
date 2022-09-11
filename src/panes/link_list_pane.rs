use super::Pane;

#[derive(Debug)]
pub struct LinkListPane {
    links: Vec<Link>,
}

impl LinkListPane {
    pub fn new(links: Vec<Link>) -> Self {
        Self { links }
    }
}

impl Pane for LinkListPane {
    fn render_html(&self) -> String {
        let mut r = String::new();

        for link in &self.links {
            r.extend(link.render_html().drain(..));
        }

        r
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
