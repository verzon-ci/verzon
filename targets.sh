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
checksum_ending=".sha256"
name="verzon"

rm -r dist
mkdir -p dist

# Clean previous build artifacts to avoid GLIBC version conflicts
cargo clean

for target in "${targets[@]}"; do
  echo "Compiling for $target"
  cross build --target $target --release

  path="target/$target/release/$name"


  if [ -f $path$windows_ending ]; then
    resolved_path=$path$windows_ending

    checksum=$(sha256sum $resolved_path | cut -d ' ' -f 1)
    echo $checksum > dist/$name-$target$windows_ending$checksum_ending

    mv $resolved_path dist/$name-$target$windows_ending
    continue
  fi

  if [ -f $path ]; then
    checksum=$(sha256sum $path | cut -d ' ' -f 1)
    echo $checksum > dist/$name-$target$checksum_ending

    mv $path dist/$name-$target
    continue
  fi
done
