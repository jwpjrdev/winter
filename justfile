build:
    cargo build
    sudo cp /workspace/winter/target/debug/winter /usr/local/bin/

test: build
    cargo test

clean:
    cargo clean

release:
    cargo build --release
    sudo cp /workspace/winter/target/debug/winter /usr/local/bin/
