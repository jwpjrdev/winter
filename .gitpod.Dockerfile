FROM gitpod/workspace-full

# update rust? it might already be up to date though
RUN rustup toolchain uninstall stable && rustup toolchain install stable