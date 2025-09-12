default:
  just --list

# Run the application
run FILE_PATH="assets/demo.npy":
  cargo run -- {{FILE_PATH}}

run-float FILE_PATH="assets/demo_float.npy":
  cargo run -- {{FILE_PATH}}

# Create a .npy file with int values using the provided script (used for testing)
npy_create FILE_PATH="assets/demo.npy":
  uv run ./scripts/npy_create.py {{FILE_PATH}}

# Create a .npy with float values file using the provided script (used for testing)
npy_create_float FILE_PATH="assets/demo_float.npy":
  uv run ./scripts/npy_create.py {{FILE_PATH}} --float

# Run all steps from the CI
ci: check format lint build run run-float doc

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

# Run all tests
test:
  cargo test --all-features
