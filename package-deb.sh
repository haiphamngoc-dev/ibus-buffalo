#!/usr/bin/env bash
#
# IBus Buffalo - Debian Packaging Script
# Copyright (C) 2026 Hai Pham Ngoc
#
set -e

# Configuration
PKG_NAME="ibus-buffalo"
RAW_VERSION="${1:-0.1.0}"
VERSION="${RAW_VERSION#v}"
ARCH="amd64"
STAGE_DIR="target/debian/${PKG_NAME}_${VERSION}_${ARCH}"

echo "=== Step 1: Building release binaries ==="
make build

echo "=== Step 2: Preparing staging directory ==="
rm -rf "target/debian"
mkdir -p "${STAGE_DIR}/DEBIAN"

echo "=== Step 3: Installing files to staging directory ==="
make install DESTDIR="${STAGE_DIR}"

echo "=== Step 4: Generating DEBIAN/control file ==="
cat <<EOF > "${STAGE_DIR}/DEBIAN/control"
Package: ${PKG_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: ibus, libgtk-4-1, libx11-6, libxtst6, libc6
Maintainer: Hai Pham Ngoc
Description: Vietnamese Input Method Engine for IBus (Buffalo)
 A clean, lightweight, and modern Vietnamese input method engine for IBus,
 built in Rust and GTK4.
EOF

echo "=== Step 5: Packaging using dpkg-deb ==="
dpkg-deb --root-owner-group --build "${STAGE_DIR}" "target/debian/${PKG_NAME}_${VERSION}_${ARCH}.deb"

echo "=== Success ==="
echo "Debian package successfully created at:"
echo "  target/debian/${PKG_NAME}_${VERSION}_${ARCH}.deb"
echo ""
echo "You can install it using: sudo dpkg -i target/debian/${PKG_NAME}_${VERSION}_${ARCH}.deb"
