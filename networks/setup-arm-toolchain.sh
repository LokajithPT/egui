#!/bin/bash

TOOLCHAIN_DIR="$HOME/arm-toolchain"
TOOLCHAIN_URL="https://releases.linaro.org/components/toolchain/binaries/7.5-2019.12/arm-linux-gnueabihf/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz"
TOOLCHAIN_FILE="/tmp/gcc-linaro-7.5.0-2019.12-x86_64_arm-linux-gnueabihf.tar.xz"

if [ -d "$TOOLCHAIN_DIR/bin" ]; then
    echo "Toolchain already exists at $TOOLCHAIN_DIR"
else
    echo "Downloading Linaro ARM toolchain..."
    wget -O "$TOOLCHAIN_FILE" "$TOOLCHAIN_URL"

    echo "Extracting..."
    mkdir -p "$TOOLCHAIN_DIR"
    tar -xf "$TOOLCHAIN_FILE" -C "$TOOLCHAIN_DIR" --strip-components=1

    rm -f "$TOOLCHAIN_FILE"
    echo "Toolchain installed to $TOOLCHAIN_DIR"
fi

export PATH="$TOOLCHAIN_DIR/bin:$PATH"
echo "ARM toolchain ready. Run: export PATH=\"$TOOLCHAIN_DIR/bin:\$PATH\""