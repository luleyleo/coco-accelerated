#!/usr/bin/env bash

set -e

nvidia-smi

cargo -V
rustfmt -V
futhark -V

cargo criterion --features "reference c cuda"
