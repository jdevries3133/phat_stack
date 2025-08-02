use flate2::{Compression, write::GzEncoder};
use std::{
    env::{current_dir, var},
    fs::{File, remove_dir_all},
    path::Path,
};

fn main() {
    let template_dir = current_dir()
        .expect("can view current working directory")
        .parent()
        .expect("current dir has a parent")
        .join("phat-service-template");
    if !template_dir.exists() {
        println!(
            "cargo::error=cannot find phat-service-template at path {template_dir:?}"
        );
        return;
    }
    let node_modules = template_dir.join("node_modules");
    if node_modules.exists() {
        remove_dir_all(&node_modules).expect("removed node_modules from template dir to avoid packaging into CLI");
    }
    let out_dir = var("OUT_DIR").expect("OUT_DIR is set");
    let workdir = Path::new(&out_dir);
    let tarball = workdir.join("template.tgz");
    let tarball = File::create(tarball).expect("can open tarball for writing");
    let enc = GzEncoder::new(tarball, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", template_dir)
        .expect("can append_dir_all template tarball");
    tar.finish().expect("can finish template tarball");
    println!("cargo::rerun-if-changed=**/*");
    println!("cargo::rerun-if-changed=../phat-service-template/**/*");
}
