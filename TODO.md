# TODO

- [ ] in `./crates/create-phat-app/build.rs`, there is a bug where it doesn't
  emit all of the `cargo::rerun-if-changed` directives needed to be responsive
  to changes in `./crates/phat-service-template`. This can be automated by
  walking the file-tree beneath `./crates/phat-service-template`, and emitting a
  `rerun-if-changed` command for each file in the template

# Done
