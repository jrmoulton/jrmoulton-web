#!/bin/bash

cargo r -r --bin gen;
sudo setcap CAP_NET_BIND_SERVICE=+eip ./target/release/web
cargo r -r --bin web
