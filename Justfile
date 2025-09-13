default:
  just --list

# Run the application
run FILE_PATH="assets/demo.npy":
  cargo run -- {{FILE_PATH}}

# Create .npy files according to scripts/npy_definitions.json
npy-create:
  uv run ./scripts/npy_create.py

# Run all steps from the CI
ci: npy-create check format lint build test doc

# Run cargo check
check:
  cargo check --all-targets --all-features

# Format all files
format: format-rust format-rest

# Format the code with rustfmt
format-rust:
  cargo fmt --all

# Format all other files with dprint
format-rest:
  dprint fmt

# Run clippy linter
lint:
  cargo clippy --all-targets --all-features -- -D warnings

# Generate documentation
doc:
  cargo doc --no-deps --all-features

# Generate documentation and open it in the browser
doc-open:
  just doc && open target/doc/git_local_review/index.html

# Build the application
build:
  cargo build

# Build all release artifacts
build-release:
  ./scripts/build_release.sh

# Run all tests
test:
  cargo test --all-features
