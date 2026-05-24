#!/bin/bash

set -e

MANIFEST="io.github.relativemodder.tetrio-flatpak.json"
BUILD_DIR="build-dir"
REPO_DIR="repo"

flatpak run \
    --filesystem=host \
    --socket=wayland \
    --socket=fallback-x11 \
    --share=ipc \
    --device=dri \
    --socket=pulseaudio \
    --share=network \
    org.flatpak.Builder \
    --force-clean \
    --install-deps-from=flathub \
    --repo="$REPO_DIR" \
    "$BUILD_DIR" \
    "$MANIFEST"

flatpak --user install --or-update -y ./repo io.github.relativemodder.tetrio-flatpak
