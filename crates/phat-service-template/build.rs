use std::{fs, path::Path, process::Command};

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

        fs::write(&htmx_file, &htmx_source).expect("Failed to write HTMX file");
    }

    let output = Command::new("openssl")
        .args(["dgst", "-sha256", "-hex", &htmx_file])
        .output()
        .expect("Failed to execute openssl command");

    if !output.status.success() {
        panic!("Failed to calculate checksum for {htmx_file}");
    }

    let checksum_output = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 in checksum output");

    let actual_checksum =
        checksum_output.split('=').nth(1).unwrap_or("").trim();

    if actual_checksum != HTMX_CHECKSUM {
        let _ = fs::remove_file(&htmx_file);
        panic!(
            "HTMX checksum mismatch!\nExpected: {HTMX_CHECKSUM}\nActual: {actual_checksum}"
        );
    }
    println!("cargo:rerun-if-changed={htmx_file}");
}
