#!/bin/bash
# ==============================================================================
# JettraRDB - Build and Packaging Helper Script
# ==============================================================================
set -e

echo "Building JettraRDB in Release Mode..."
cargo build --release

echo ""
echo "Compilation complete!"
echo "Binary location: target/release/jettrardb"
ls -lh target/release/jettrardb 2>/dev/null || true

