use std::convert::Infallible;

use frontpage::FrontPage;
use warp::Filter;

mod frontpage;
mod panes;
mod providers;

#[tokio::main]
async fn main() {
    let page = warp::path!().and_then(front_page_render);

    let download = warp::path!("download.mobi").and_then(download_render);

    warp::serve(page.or(download))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn front_page_render() -> Result<impl warp::Reply, Infallible> {
    let fp = FrontPage::new();
    let s = fp.render().await;

    Ok(warp::reply::html(s))
}

async fn download_render() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html("testing".to_owned()))
}
