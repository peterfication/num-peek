#!/usr/bin/env bash
set -euo pipefail

PROJECT_NAME=$(basename "$(pwd)")
# Extract version from Cargo.toml
VERSION=$(grep '^version\s*=' Cargo.toml | head -n1 | sed -E 's/version\s*=\s*"([^"]+)"/\1/')
DIST_DIR="dist"
mkdir -p "$DIST_DIR"

TARGETS=(
  "x86_64-apple-darwin"
  "aarch64-apple-darwin"
  "x86_64-unknown-linux-gnu"
  "aarch64-unknown-linux-gnu"
)

for TARGET in "${TARGETS[@]}"; do
  echo "Building for $TARGET..."
  if rustup target list --installed | grep -q "^$TARGET$"; then
    echo "Target $TARGET already installed."
  else
    if ! rustup target add "$TARGET"; then
      echo "Error: Failed to add target $TARGET" >&2
      exit 1
    fi
  fi

  # Set up cross-compilation for Linux targets on macOS
  if [[ "$TARGET" == *linux* ]]; then
    echo "Setting up cross-compilation for $TARGET..."
    if ! command -v cross &>/dev/null; then
      cargo install cross
    fi
    cross build --release --target "$TARGET"
  else
    cargo build --release --target "$TARGET"
  fi

  BIN_PATH="target/$TARGET/release/$PROJECT_NAME"
  PKG_NAME="${PROJECT_NAME}-v${VERSION}-${TARGET}.tar.gz"
  PKG_PATH="$DIST_DIR/$PKG_NAME"

  echo "Packaging $PKG_NAME..."
  tar -czf "$PKG_PATH" -C "target/$TARGET/release" "$PROJECT_NAME"

  # Calculate SHA256 and save
  SHASUM=$(shasum -a 256 "$PKG_PATH" | awk '{print $1}')
  echo "$SHASUM  $PKG_NAME" > "$PKG_PATH.sha256"
done

echo "All builds complete. Packages are in $DIST_DIR/"
