# Networks - BeagleBone TCP Server

Cross-compiled Rust TCP server for BeagleBone Black running ARM.

## Build

### 1. Setup ARM toolchain
```bash
./setup-arm-toolchain.sh
```

### 2. Build for ARM
```bash
CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER="$HOME/arm-toolchain/bin/arm-linux-gnueabihf-gcc" cargo build --release --target=arm-unknown-linux-gnueabihf
```

### 3. Output binary
```
target/arm-unknown-linux-gnueabihf/release/networks
```

## Deploy to BeagleBone

```bash
scp target/arm-unknown-linux-gnueabihf/release/networks ubuntu@192.168.7.2:~/
```

## Run on BeagleBone

```bash
ssh ubuntu@192.168.7.2
./networks
```

Server listens on `192.168.7.2:8080`

## Test

```bash
echo "vers" | nc -w 2 192.168.7.2 8080
# Output: Version is 1.11

echo "on" | nc -w 2 192.168.7.2 8080
# Output: System turned ON

echo "off" | nc -w 2 192.168.7.2 8080
# Output: System turned OFF
```

## Check if running on BeagleBone

```bash
ssh ubuntu@192.168.7.2 "ss -tlnp | grep 8080"
```

## Commands

- `vers` - Get version
- `on` - Turn system on
- `off` - Turn system off