# Release a new version

- Update [`CHANGELOG.md`](CHANGELOG.md)
- Update version in [`Cargo.toml`](Cargo.toml)
- Merge and tag the new version
- Build the new release `just build-release`
- On Github, convert the tag into a release and upload the binaries and SHA files
- Update the Homebrew `Formula` in [peterfication/homebrew-num-peek](https://github.com/peterfication/homebrew-num-peek)
