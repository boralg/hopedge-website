use askama::Template;
use pagebake::redirects::RedirectList;
use pagebake::{get, render::RenderConfig, Router};
use std::fs;
use std::path::Path;
use templates::{Main, NotFound};

mod data;
mod raw;
mod templates;

fn main() {
    let router = Router::new()
        .route("/", get(|| Main::default().render().unwrap()))
        .fallback(|| NotFound::default().render().unwrap());

    router
        .render(
            Path::new("public"),
            RenderConfig {
                redirect_lists: vec![
                    RedirectList::for_cloudflare_pages(),
                    RedirectList::for_static_web_server(),
                ],
                ..Default::default()
            },
        )
        .unwrap();

    let public_assets_path = Path::new("public/assets");
    if public_assets_path.exists() {
        fs::remove_dir_all(public_assets_path).unwrap();
    }

    fs_extra::dir::copy("assets", "public", &fs_extra::dir::CopyOptions::new()).unwrap();
}
