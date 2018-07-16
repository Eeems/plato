#!/bin/bash
set -ex

./build.sh
./dist.sh

DEVICE_IP="192.168.86.179"
ssh root@$DEVICE_IP 'systemctl stop xochitl || true ; killall -9 plato || true; killall -9 draft || true; systemctl stop xochitl || true'
scp -r ./dist root@$DEVICE_IP:~/
ssh root@$DEVICE_IP 'cd dist && RUST_BACKTRACE=1 PRODUCT=remarkable LD_LIBRARY_PATH=./libs ./plato ' "$@"