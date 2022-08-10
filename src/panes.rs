mod link_list_pane;

pub use link_list_pane::{Link, LinkListPane};

pub trait Pane {
    fn render_html(&self) -> String;
}
