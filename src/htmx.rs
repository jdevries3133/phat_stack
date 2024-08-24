//! HTMX utils

use axum::http::{HeaderMap, HeaderValue};
#[cfg(feature = "stripe")]
use axum::response::{IntoResponse, Redirect};

/// Inserts a `Hx-Redirect` header into the provided headers. Will panic if
/// `to` cannot be encoded as an [axum::http::HeaderValue].
pub fn redirect(mut headers: HeaderMap, to: &str) -> HeaderMap {
    headers.insert(
        "Hx-Redirect",
        HeaderValue::from_str(to)
            .unwrap_or(HeaderValue::from_str("/").unwrap()),
    );
    headers
}

/// Like the above, but better
#[cfg(feature = "stripe")]
pub fn redirect_2(headers: HeaderMap, to: &str) -> impl IntoResponse {
    let headers = redirect(headers, to);
    let response = Redirect::to(to);
    (headers, response)
}

pub const fn get_client_script() -> &'static str {
    concat!(
        include_str!("./htmx-2.0.2.vendor.js"),
        r#"
        function makeId(length) {
            let result = '';
            const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
            const charactersLength = characters.length;
            let counter = 0;
            while (counter < length) {
              result += characters.charAt(Math.floor(Math.random() * charactersLength));
              counter += 1;
            }
            return result;
        }

        htmx.on('htmx:responseError', () => {
            const el = document.createElement('p');
            const id = makeId(20);
            el.id = id;
            el.innerText = "An error occurred; sorry for the inconvenience! (click to dismiss)";
            el.classList.add("bg-red-100");
            el.classList.add("p-2");
            el.classList.add("rounded");
            el.classList.add("w-full");
            el.classList.add("sticky");
            el.classList.add("top-0");
            el.classList.add("dark:text-black");
            el.classList.add("cursor-pointer");
            document.body.insertBefore(el, document.body.firstChild);
            el.addEventListener('click', () => {
                document.getElementById(id).remove();
            });
        });

        htmx.on('htmx:beforeSwap', (e) => {
            if (e.detail.xhr.status === 400) {
                e.detail.shouldSwap = true;
                e.detail.isError = false;
            }
        })

        htmx.config.defaultSwapStyle = "outerHTML";
    "#
    )
}
