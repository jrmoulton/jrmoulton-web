#!/bin/bash

# cargo r -r --bin gen;
# cargo b -r --bin web
sudo setcap CAP_NET_BIND_SERVICE=+eip ./target/release/web
./target/release/web
