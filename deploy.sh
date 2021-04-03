#!/bin/bash

cargo build --release
cp -f /home/mathuis/Downloads/tcp_proxy/target/release/tcp_proxy /home/mathuis/Downloads/LiveCapital/ApplicationData/requests/tcp_proxy