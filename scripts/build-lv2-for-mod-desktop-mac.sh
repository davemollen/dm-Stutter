#!/bin/bash
binary_name_to_replace="libdm_stutter.so"
binary_name="libdm_stutter.dylib"
lv2_folder="dm-Stutter.lv2"
move_to="$lv2_folder/$binary_name"

# compile binary
cd lv2
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --release --target x86_64-apple-darwin
MACOSX_DEPLOYMENT_TARGET=10.15 cargo build --release --target aarch64-apple-darwin
lipo -create target/x86_64-apple-darwin/release/$binary_name target/aarch64-apple-darwin/release/$binary_name -output target/release/$binary_name
file target/release/$binary_name

# move compiled binary
if [ -d "$move_to" ]; then
    rm -r "$move_to"
fi
if mv target/release/$binary_name $move_to; then
    echo "Copied lv2 binary to $move_to"
fi

# replace <binary_name>.so with <binary_name>.dylib in manifest.ttl
sed -i '' "s/$binary_name_to_replace/$binary_name/g" $lv2_folder/manifest.ttl