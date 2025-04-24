# hat-splitter

The `hat-splitter` crate implements the splitting rule described in the
[Hierarchical Autoregressive Transformers
paper](https://arxiv.org/abs/2501.10322v2). You can use this to implement
training and inference of HAT models.

## Installation

```bash
cargo add hat-splitter
```

## Usage

```rust
use hat_splitter::{HATSplitter, Splitter};

let my_hat_splitter = HATSplitter::new();
let split_text: Vec<String> = my_hat_splitter.split("Hello, world!");
assert_eq!(split_text, vec!["Hello,", " world!"]);

let split_text: Vec<Vec<u8>> = my_hat_splitter.split_with_limit("Hello, world!", 4);
assert_eq!(split_text, vec![b"Hell".to_vec(), b"o,".to_vec(), b" wor".to_vec(), b"ld!".to_vec()]);
```
