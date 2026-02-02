# Justfile for webtitle

# Binary name from Cargo.toml
binary_name := "webtitle"

# Build in release mode
build:
    cargo build --release

# Install the binary to ~/.local/bin or fallback to /usr/local/bin
install: build
    #!/usr/bin/env sh
    set -eu
    BINARY="target/release/{{binary_name}}"
    LOCAL_BIN="$HOME/.local/bin"
    SYSTEM_BIN="/usr/local/bin"

    # Try to create and use ~/.local/bin
    if mkdir -p "$LOCAL_BIN" 2>/dev/null && [ -w "$LOCAL_BIN" ]; then
        cp "$BINARY" "$LOCAL_BIN/{{binary_name}}"
        echo "Installed {{binary_name}} to $LOCAL_BIN"
        echo "Make sure $LOCAL_BIN is in your PATH"
    else
        echo "Cannot use $LOCAL_BIN, falling back to $SYSTEM_BIN (requires sudo)"
        sudo cp "$BINARY" "$SYSTEM_BIN/{{binary_name}}"
        echo "Installed {{binary_name}} to $SYSTEM_BIN"
    fi

# Clean build artifacts
clean:
    cargo clean

# Run clippy linter
lint:
    cargo clippy

# Format code
fmt:
    cargo fmt

# Run tests
test:
    cargo test
