#!/bin/bash

targets=(
  "aarch64-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
  "x86_64-unknown-linux-gnu"
  "arm-unknown-linux-gnueabi"
  "armv7-unknown-linux-gnueabi"
  "x86_64-unknown-freebsd"
)

windows_ending=".exe"
name="verzon"

rm -r dist
mkdir -p dist

for target in "${targets[@]}"; do
  echo "Compiling for $target"
  cross build --target $target --release

  path="target/$target/release/$name"

  if [ -f $path$windows_ending ]; then
    mv $path$windows_ending dist/$name-$target$windows_ending
  fi

  if [ -f $path ]; then
    mv $path dist/$name-$target
  fi
done
