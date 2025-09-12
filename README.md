# num-peek

A CLI tool that can peek into `*.npy` files.

## Development

### Prerequisites

- Rust version >= `1.88.0`
- [just](https://github.com/casey/just) task runner

### Getting started

```bash
just ci
just run
just run-float
```

## Roadmap

- Limit showing unique values to 10 values
  - If there are more than 10 values, show the smallest 10 and the highest 10
- Total Elements
- Size/memory usage
- Mean, Standard Deviation, Median
- Improve code organization
- Add CI
- Publish crate and add installation instructions

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
