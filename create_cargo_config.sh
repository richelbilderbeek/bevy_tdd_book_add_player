#!/bin/bash

# Create the folder, no warning if it already exists
mkdir --parent .cargo

# Create the file, no warning if it already exists
touch .cargo/config.toml

# Write the lines needed, will remove all previous content
echo '[target.x86_64-unknown-linux-gnu]' > .cargo/config.toml
echo 'linker = "clang"' >> .cargo/config.toml
echo 'rustflags = ["-C", "link-arg=-fuse-ld=lld"]' >> .cargo/config.toml
