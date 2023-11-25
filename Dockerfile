FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/phat_stack phat_stack
ENTRYPOINT [ "./phat_stack" ]
