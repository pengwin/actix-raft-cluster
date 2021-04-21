@echo off
title follower
cargo run -- -n 2 -h 127.0.0.1 -p 8088  --leader-id=1 --leader-addr http://127.0.0.1:8080