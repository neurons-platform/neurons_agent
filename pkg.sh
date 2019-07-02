#!/bin/bash




sudo docker exec -it rust-musl cargo build --release
mkdir -p agent/
cp config.ini agent/
cp target/x86_64-unknown-linux-musl/release/neurons_agent agent/
tar czvf agent.tar.gz agent


