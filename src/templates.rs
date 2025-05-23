use askama::Template;

use crate::{data::static_paths::StaticPaths, raw::Raw};

#[derive(Template)]
#[template(path = "main.html")]
pub struct Main {
    header: Raw<Header>,
    main_css_path: String,
    dark_mode_css_path: String,
    dark_mode_js_path: String,
}

impl Default for Main {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            header: Raw::to_raw(Header::default()),
            main_css_path: sp.main_css_path.clone(),
            dark_mode_css_path: sp.dark_mode_css_path.clone(),
            dark_mode_js_path: sp.dark_mode_js_path.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "header.html")]
pub struct Header {
    pub header_css_path: String,
    pub logo_path: String,
}

impl Default for Header {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            header_css_path: sp.header_css_path.clone(),
            logo_path: "".to_owned(), //sp.logo_path.clone(),
        }
    }
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound {
    header: Raw<Header>,
    main_css_path: String,
    dark_mode_css_path: String,
    dark_mode_js_path: String,
}

impl Default for NotFound {
    fn default() -> Self {
        let sp = StaticPaths::new();
        Self {
            header: Raw::to_raw(Header::default()),
            main_css_path: sp.main_css_path.clone(),
            dark_mode_css_path: sp.dark_mode_css_path.clone(),
            dark_mode_js_path: sp.dark_mode_js_path.clone(),
        }
    }
}
