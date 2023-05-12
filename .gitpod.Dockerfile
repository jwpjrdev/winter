FROM gitpod/workspace-full

# update rust
RUN rustup toolchain uninstall stable && rustup toolchain install stable

# install rustfmt & clippy
RUN rustup component add rustfmt clippy
