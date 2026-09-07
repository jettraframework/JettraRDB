# JettraRDB - Executables Generation & Compilation Guide

This guide explains how to compile and generate standalone, high-performance binary executables of **JettraRDB** for Linux, Windows, macOS, and ARM64 platforms.

---

## 1. Prerequisites

### Installing the Rust Toolchain (via `rustup`)
If you do not have Rust installed on your development machine or server, install it with the official installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

Verify your installation:
```bash
rustc --version
cargo --version
```

Recommended minimum version: **Rust 1.80+**.

---

## 2. Native Compilation (Host Machine)

### 2.1 Debug Build (Fast Compilation)
Ideal during active development:
```bash
cd JettraRDB
cargo build
```
The output binary will be located at:
```
target/debug/jettrardb
```

### 2.2 Production Release Build (Max Performance)
Compiles with link-time optimization (LTO), symbol stripping, and vectorization:
```bash
cargo build --release
```
The optimized production binary is generated at:
```
target/release/jettrardb
```

Test running the compiled executable:
```bash
./target/release/jettrardb
```

---

## 3. Static Musl Binaries (Portable Linux)

By compiling against **musl libc** instead of GNU libc (glibc), the generated binary is 100% statically linked. It can run on **any** Linux distribution (Ubuntu, Debian, RHEL, CentOS, Alpine, Arch) without requiring dynamic shared libraries or matching libc versions.

### 3.1 Install the Musl Target
```bash
rustup target add x86_64-unknown-linux-musl
```

Install the musl development package on Debian/Ubuntu:
```bash
sudo apt-get install -y musl-tools
```

### 3.2 Compile Static Binary
```bash
cargo build --release --target x86_64-unknown-linux-musl
```
The self-contained static executable is located at:
```
target/x86_64-unknown-linux-musl/release/jettrardb
```

Verify that the binary has no dynamic dependencies:
```bash
file target/x86_64-unknown-linux-musl/release/jettrardb
# Output should indicate: "statically linked"
ldd target/x86_64-unknown-linux-musl/release/jettrardb
# Output: "not a dynamic executable"
```

---

## 4. Cross-Compilation for Other Platforms

### 4.1 Linux ARM64 / AArch64 (Raspberry Pi, AWS Graviton, Apple Silicon VMs)
```bash
# Add target
rustup target add aarch64-unknown-linux-musl

# Install cross tool if cross-compiling from x86_64
cargo install cross

# Build for ARM64
cross build --release --target aarch64-unknown-linux-musl
```
Output:
```
target/aarch64-unknown-linux-musl/release/jettrardb
```

### 4.2 Windows Executable (`.exe`)
From Linux or macOS, you can build a native Windows `.exe`:
```bash
# Add target
rustup target add x86_64-pc-windows-gnu

# On Ubuntu/Debian, install MinGW
sudo apt-get install -y mingw-w64

# Build .exe
cargo build --release --target x86_64-pc-windows-gnu
```
Output:
```
target/x86_64-pc-windows-gnu/release/jettrardb.exe
```

### 4.3 macOS (Apple Silicon & Intel)
Using `cross` or directly on macOS:
```bash
# Apple Silicon (M1/M2/M3/M4)
cargo build --release --target aarch64-apple-darwin

# Intel Macs
cargo build --release --target x86_64-apple-darwin
```

---

## 5. Binary Size Optimization & Stripping

`JettraRDB` includes pre-configured optimization flags in `Cargo.toml`:
```toml
[profile.release]
opt-level = 3          # Maximum speed optimization
lto = true             # Link-Time Optimization across all crates
codegen-units = 1      # Max inlining and dead code elimination
panic = "abort"        # Eliminates unwinding landing pads
strip = true           # Automatically strips debug symbols
```

### Further Compression with UPX (Optional)
If you want to reduce the binary size to under **6 MB**:
```bash
sudo apt-get install -y upx
upx --best --lzma target/release/jettrardb
```

---

## 6. Running as a Linux Systemd Service

To deploy JettraRDB as a managed background system service:

1. Copy the release binary to `/usr/local/bin`:
```bash
sudo cp target/release/jettrardb /usr/local/bin/jettrardb
sudo chmod +x /usr/local/bin/jettrardb
```

2. Create a dedicated database directory and user:
```bash
sudo useradd -r -s /bin/false jettra
sudo mkdir -p /var/lib/jettrardb
sudo chown -R jettra:jettra /var/lib/jettrardb
```

3. Create the systemd service file:
```ini
# /etc/systemd/system/jettrardb.service
[Unit]
Description=JettraRDB High-Density Multi-Model Storage Engine
After=network.target

[Service]
Type=simple
User=jettra
Group=jettra
WorkingDirectory=/var/lib/jettrardb
Environment="JETTRA_DATA_DIR=/var/lib/jettrardb/data"
Environment="JETTRA_DB_PORT=8086"
Environment="JETTRA_GUI_PORT=50050"
Environment="JETTRA_GRPC_PORT=50051"
ExecStart=/usr/local/bin/jettrardb
Restart=always
RestartSec=5
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

4. Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now jettrardb
sudo systemctl status jettrardb
```

