# num-peek

A CLI tool that can peek into `*.npy` files.

## Installation

```bash
cargo install num-peek
```

## Usage example

```bash
$ num-peek assets/demo.npy
Peek into assets/demo.npy
----------------------------------------
Dimensions: 2
Shape: [2, 3]
Type: Int8
----------------------------------------
Number of unique values: 6
Unique values: [1, 3, 4, 8, 12, 22]
Min value: 1
Max value: 22
```

### Yazi previewer

`num-peek` can be used together with [`piper`](https://github.com/yazi-rs/plugins/tree/main/piper.yazi) as a [Yazi](https://yazi-rs.github.io/) previewer:

```toml
# yazi.toml
[plugin]
prepend_previewers = [
  { name = "*.npy", run = 'piper -- num-peek $1' },
]
```

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
- Mean, Standard Deviation, Median
- Different output formats: text, json, ...

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
