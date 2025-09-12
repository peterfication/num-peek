default:
  just --list

# Run the application
run:
  cargo run

# Create a .npy file using the provided script (used for testing)
npy_create FILE_PATH="assets/demo.npy":
  uv run ./scripts/npy_create.py {{FILE_PATH}}
