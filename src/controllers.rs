use crate::{components, htmx, prelude::*};
use anyhow::Result;
use axum::{
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
#[cfg(feature = "live_reload")]
use serde::Deserialize;

pub async fn root() -> impl IntoResponse {
    components::Page {
        title: "PHAT Stack",
        children: &components::Home {},
    }
    .render()
}

#[cfg(feature = "live_reload")]
#[derive(Deserialize)]
pub struct PongParams {
    pub poll_interval_secs: u64,
}
/// The client will reload when this HTTP long-polling route disconnects,
/// effectively implementing live-reloading.
#[axum_macros::debug_handler]
#[cfg(feature = "live_reload")]
pub async fn pong(
    axum::extract::Query(PongParams { poll_interval_secs }): axum::extract::Query<PongParams>,
) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_secs(poll_interval_secs))
        .await;
    "pong"
}

#[cfg(not(feature = "live_reload"))]
pub async fn pong() -> impl IntoResponse {
    "pong"
}

/// You may be wondering why this sits on a separate response while the
/// tailwind styles are inlined into the page template and basically
/// hard-coded into every initial response. This is because the CSS is a
/// blocker for page rendering, so we want it right there in the initial
/// response. Meanwhile, it's fine for the browser to fetch and run HTMX
/// asynchronously since it doesn't really need to be on the page until the
/// first user interaction.
///
/// Additionally, our HTMX version does not change very often. We can exploit
/// browser cachine to mostly never need to serve this resource, making the
/// app more responsive and cutting down on overall bandwidth. That's also why
/// we have the HTMX version in the URL path -- because we need to bust the
/// browser cache every time we upgrade.
pub async fn get_htmx_js() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("text/javascript")
            .expect("We can insert text/javascript headers"),
    );
    headers.insert(
        "Cache-Control",
        HeaderValue::from_str("public, max-age=31536000")
            .expect("we can set cache control header"),
    );

    (headers, htmx::get_client_script())
}

pub async fn get_favicon() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("image/x-icon")
            .expect("We can insert image/x-icon header"),
    );
    (headers, include_bytes!("./static/favicon.ico"))
}

fn png_controller(bytes: &'static [u8]) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("image/png")
            .expect("We can insert image/png header"),
    );
    (headers, bytes)
}

pub async fn get_tiny_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/icon-16x16.png"))
}

pub async fn get_small_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/icon-32x32.png"))
}

pub async fn get_medium_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/icon-192x192.png"))
}

pub async fn get_large_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/icon-512x512.png"))
}

pub async fn get_maskable_small_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/maskable_icon_x72.png"))
}

pub async fn get_maskable_medium_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/maskable_icon_x128.png"))
}

pub async fn get_maskable_large_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/maskable_icon_x192.png"))
}

pub async fn get_apple_icon() -> impl IntoResponse {
    png_controller(include_bytes!("./static/apple-touch-icon.png"))
}

pub async fn get_manifest() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/manifest+json")
            .expect("We can insert application/manifest+json header"),
    );
    (headers, include_str!("./static/manifest.json"))
}

pub async fn get_robots_txt() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("text/plain")
            .expect("We can insert text/plain header"),
    );
    (headers, "# beep boop\nUser-agent: *\nAllow: /")
}

pub async fn user_home(
    headers: HeaderMap,
) -> Result<impl IntoResponse, ServerError> {
    let session = Session::from_headers_err(&headers, "user home")?;
    let html = components::Page {
        title: "Home Page",
        children: &components::PageContainer {
            children: &components::UserHome {
                username: &session.username,
            },
        },
    }
    .render();

    Ok(html)
}

pub async fn void() -> &'static str {
    ""
}

pub async fn about() -> impl IntoResponse {
    components::Page {
        title: "About",
        children: &components::PageContainer {
            children: &components::AboutPage {},
        },
    }
    .render()
}
