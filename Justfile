test:
    cargo test

install:
    cargo install --path .

run *ARGS:
    cargo run -- "@$"