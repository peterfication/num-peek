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
