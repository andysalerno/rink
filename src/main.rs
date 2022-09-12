use std::convert::Infallible;

use frontpage::FrontPage;
use reqwest::Url;
use warp::Filter;

mod frontpage;
mod panes;
mod providers;

#[tokio::main]
async fn main() {
    let page = warp::path!().and_then(front_page_render);

    let download_mobi = warp::path!("download.mobi").and_then(download_render);

    let download = warp::path!("download" / String).and_then(download_page);

    warp::serve(page.or(download_mobi).or(download))
        .run(([0, 0, 0, 0], 3030))
        .await;
}

async fn front_page_render() -> Result<impl warp::Reply, Infallible> {
    let fp = FrontPage::new();
    let s = fp.render().await;

    Ok(warp::reply::html(s))
}

async fn download_render() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::http::response::Response::builder()
        .header("Content-Type", "application/x-mobipocket-ebook")
        .body("blahblahblah"))
}

async fn download_page(url: String) -> Result<impl warp::Reply, Infallible> {
    let url = url.replace("%2F", "/");
    let url = url.replace(".mobi", "");
    println!("downloading: {url}");
    let content = reqwest::get(url).await.unwrap();
    let content = content.text().await.unwrap();

    Ok(warp::http::response::Response::builder()
        .header("Content-Type", "application/x-mobipocket-ebook")
        .body(content))
}
