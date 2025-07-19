use aws_lc_rs::digest;
use std::{fs, path::Path};

const HTMX_VERSION: &str = "2.0.2";
const HTMX_CHECKSUM: &str =
    "e1746d9759ec0d43c5c284452333a310bb5fd7285ebac4b2dc9bf44d72b5a887";

fn main() {
    let htmx_file = format!("src/htmx-{HTMX_VERSION}.vendor.js");

    if !Path::new(&htmx_file).exists() {
        let url = format!("https://unpkg.com/htmx.org@{HTMX_VERSION}");
        let mut response =
            ureq::get(&url).call().expect("can fetch htmx from unpkg");
        if response.status() != ureq::http::StatusCode::OK {
            panic!(
                "received non-200 response from unpkg: {}",
                response.status()
            );
        }
        let htmx_source = response
            .body_mut()
            .read_to_string()
            .expect("can read HTMX source as a string");

        let digest_result =
            digest::digest(&digest::SHA256, htmx_source.as_bytes());
        let actual_checksum =
            hex::encode(digest_result.as_ref()).to_lowercase();

        if actual_checksum != HTMX_CHECKSUM {
            panic!(
                "HTMX checksum mismatch!\nExpected: {HTMX_CHECKSUM}\nActual: {actual_checksum}"
            );
        }

        fs::write(&htmx_file, &htmx_source).expect("Failed to write HTMX file");
    }
    println!("cargo:rerun-if-changed={htmx_file}");
}
