FROM gitpod/workspace-full

USER gitpod

RUN bash -cl "rustup toolchain install nightly"
