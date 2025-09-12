# num-peek

A CLI tool that can peek into `*.npy` files.

## Example

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
- Publish crate and add installation instructions

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))
