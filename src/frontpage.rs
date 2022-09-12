use crate::providers::{
    nyt_most_viewed_provider::NytWireProvider, nyt_wire_provider::NytMostViewedProvider,
    ContentProvider,
};

const REFRESH_TIME_SEC: usize = 1 * 60;

pub struct FrontPage {
    providers: Vec<Box<dyn ContentProvider + Send + Sync>>,
}

impl FrontPage {
    pub fn new() -> Self {
        let api_key = std::env::var("NYT_API_KEY").unwrap_or_default();

        Self {
            providers: vec![
                Box::new(NytWireProvider::new(&api_key)),
                Box::new(NytMostViewedProvider::new(&api_key)),
            ],
        }
    }

    pub async fn render(&self) -> String {
        let mut rendered = String::new();

        let mut style = pane_style();

        rendered.push_str("<!DOCTYPE html>\n");
        rendered.push_str("<html>\n");
        rendered.push_str("<head>\n");
        // rendered.push_str(&format!(
        //     "<meta http-equiv='refresh' content='{REFRESH_TIME_SEC}'>"
        // ));
        rendered.push_str("<title>testing</title>\n");
        rendered.push_str("<style>\n");
        rendered.extend(style.drain(..));
        rendered.push_str("</style>\n");

        // add fullscreen script
        let script_section = full_screen_script();
        rendered.push_str(&script_section);

        rendered.push_str("</head>\n");

        rendered.push_str("<div class='pane-parent' id='parent'>");

        // First pane
        let pane = render_provider(self.providers[0].as_ref()).await;
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Second pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Third pane pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Fourth pane pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Fifth pane pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Sixth pane pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Seventh pane pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // most viewed pane
        let pane = render_provider(self.providers[1].as_ref()).await;
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Eighth pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Nineth pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        // Tenth pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str("empty text");
        rendered.push_str("</div>\n");

        // Tenth pane
        rendered.push_str("<div class='pane'>\n");
        rendered.push_str(&pane);
        rendered.push_str("</div>\n");

        rendered.push_str("</div>");

        rendered.push_str(&modal());

        rendered.push_str("</html>");

        rendered
    }
}

async fn render_provider(provider: &dyn ContentProvider) -> String {
    let mut html = String::new();

    let pane = provider.build_pane().await;

    html.extend(pane.render_html().drain(..));

    if let Some(download_path) = pane.download_path() {
        let download_path = download_path.replace('/', "%2F");
        html.push_str(&format!(
            "<button onclick=\"location.href='/download/{download_path}'\" type='button'>download</button>"
        ));
        // html.push_str("<button onclick='location.href='http://www.example.com'" type="button">
        // www.example.com</button>");
    }

    html
}

fn pane_style() -> String {
    include_str!("frontpage.css").into()
}

fn modal() -> String {
    include_str!("modal.html").into()
}

fn full_screen_script() -> String {
    let s = include_str!("js.js");

    format!(
        "<script>
        {s}
        </script>"
    )
}
