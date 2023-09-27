# syntax = docker/dockerfile:1.4

FROM purtontech/rust-on-nails-devcontainer:1.1.12 AS builder

# set the workdir and copy the source into it
WORKDIR /home/vscode/

# OpenSSL
RUN sudo apt-get update \
    && sudo apt-get install -y \
    openssl \
    pkg-config \
    libssl-dev

# Clone the repo
RUN git clone --recurse-submodules --depth 1 https://github.com/rustformers/llm.git
# do a release build
WORKDIR /home/vscode/llm
RUN <<EOT
cargo build --release --bin llm
strip target/release/llm
EOT

# Use a slim image
FROM rust:slim-bookworm
ADD --link https://huggingface.co/rustformers/mpt-7b-ggml/resolve/main/mpt-7b-chat-q5_1-ggjt.bin /home/vscode/

# copy the binary into the final image
COPY --link --from=builder /home/vscode/llm/target/release/llm .

# set the binary as entrypoint
ENTRYPOINT ["/llm",  "mpt", "chat", "--model-path", "/mpt-7b-chat-q5_1-ggjt.bin"]
