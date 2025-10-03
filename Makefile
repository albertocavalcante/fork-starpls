# Starpls Development Makefile

.PHONY: build test fmt clean run dev-build check install uninstall

# Default target
all: build

# Build the project
build:
	bazel build //...

# Build starpls binary specifically
build-starpls:
	bazel build //crates/starpls:starpls

# Check compilation with clippy and rustfmt (fast feedback)
check:
	bazel build //...

# Run tests
test:
	bazel test //...

# Format code using hermetic rustfmt
fmt:
	bazel run @rules_rust//:rustfmt

# Sync and repin crate dependencies after adding new dependencies
# See: https://bazelbuild.github.io/rules_rust/crate_universe_bzlmod.html
sync-deps:
	CARGO_BAZEL_REPIN=1 bazel sync --only=crates --enable_workspace

# Run hermetic cargo
cargo:
	bazel run //:cargo

# Run hermetic cargo check (fast compilation check)
cargo-check:
	bazel run //:cargo -- check

# Clean build artifacts
clean:
	bazel clean

# Development build (optimized)
dev-build:
	bazel build -c opt //crates/starpls:starpls

# Quick development cycle - format and build
dev: fmt build-starpls

# Run starpls with common development flags
run:
	bazel run //crates/starpls:starpls -- server

# Variables for installation paths
PREFIX ?= $(HOME)/.local
BINDIR = $(PREFIX)/bin

# Install starpls to local bin directory
install: dev-build
	@echo "Installing starpls to $(BINDIR)/starpls..."
	@mkdir -p $(BINDIR)
	@BAZEL_BIN=$$(bazel info bazel-bin) && \
	BINARY_PATH="$$BAZEL_BIN/crates/starpls/starpls" && \
	TARGET="$(BINDIR)/starpls" && \
	cp "$$BINARY_PATH" "$$TARGET.tmp" && \
	chmod +x "$$TARGET.tmp" && \
	mv "$$TARGET.tmp" "$$TARGET" && \
	echo "✓ starpls installed successfully!"
	@echo "Make sure $(BINDIR) is in your PATH"

# Uninstall starpls from local bin directory
uninstall:
	@rm -f $(BINDIR)/starpls
	@echo "✓ starpls uninstalled"

# Help target
help:
	@echo "Available targets:"
	@echo "  build         - Build all targets"
	@echo "  build-starpls - Build starpls binary only"
	@echo "  check         - Check with clippy and rustfmt"
	@echo "  test          - Run all tests"
	@echo "  fmt           - Format code with rustfmt"
	@echo "  sync-deps     - Sync and repin crate dependencies"
	@echo "  cargo         - Run hermetic cargo"
	@echo "  cargo-check   - Run hermetic cargo check"
	@echo "  clean         - Clean build artifacts"
	@echo "  dev-build     - Optimized build"
	@echo "  dev           - Format and build (development cycle)"
	@echo "  run           - Run starpls server"
	@echo "  install       - Install starpls to \$$PREFIX/bin (default: ~/.local/bin)"
	@echo "  uninstall     - Remove starpls from \$$PREFIX/bin"
	@echo "  help          - Show this help"