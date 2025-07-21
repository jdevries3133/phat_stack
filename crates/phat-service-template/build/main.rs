#![feature(exit_status_error)]

use aws_lc_rs::digest;
use std::{fs, io::ErrorKind, path::Path, process, thread};

mod config;

fn download_htmx() {
    let htmx_file = format!("src/htmx-{}.vendor.js", config::HTMX_VERSION);

    if !Path::new(&htmx_file).exists() {
        let url =
            format!("https://unpkg.com/htmx.org@{}", config::HTMX_VERSION);
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

        if actual_checksum != config::HTMX_CHECKSUM {
            panic!(
                "HTMX checksum mismatch!\nExpected: {}\nActual: {}",
                config::HTMX_CHECKSUM,
                actual_checksum
            );
        }

        fs::write(&htmx_file, &htmx_source).expect("Failed to write HTMX file");
    }
}

fn download_node_modules() {
    if Path::new("node_modules").exists() {
        return;
    }
    let spawn_result = process::Command::new("pnpm")
        .arg("install")
        .arg("--frozen-lockfile")
        .spawn();

    match spawn_result {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!(
                    "cargo::error=pnpm CLI not found on your machine. Is it installed?"
                );
            } else {
                println!(concat!(
                    "cargo::error=pnpm install --frozen-lockfile failed for an ",
                    "unknown reason. Try running this command and looking into ",
                    "possible issues with your Node.js environment."
                ));
            }
        }
        Ok(mut process) => {
            if process
                .wait()
                .expect("can wait for install result")
                .exit_ok()
                .is_err()
            {
                println!("cargo::error=pnpm install --frozen-lockfile failed");
            }
        }
    }
}

fn do_node_build() {
    process::Command::new("pnpm")
        .arg("run")
        .arg("build")
        .spawn()
        .expect("can spawn pnpm run build")
        .wait()
        .expect("can wait on pnpm run build")
        .exit_ok()
        .expect("pnpm run build succeeded.");
}

fn main() {
    let htmx_handle = thread::spawn(download_htmx);
    let node_modules_handle = thread::spawn(download_node_modules);

    node_modules_handle
        .join()
        .expect("node_modules thread does not panic");

    do_node_build();

    htmx_handle.join().expect("htmx thread does not panic");
}
