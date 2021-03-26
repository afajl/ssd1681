#!/bin/bash

rsync -avz --delete --exclude target . pi:driver
ssh -t pi "cd driver\
  && cargo build --examples \
  && sudo env RUST_BACKTRACE=1 target/debug/examples/adafruit_1in54_tricolor"
