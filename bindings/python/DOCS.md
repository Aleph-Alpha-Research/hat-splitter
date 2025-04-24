# hat-splitter

The `hat-splitter` package implements the splitting rule described in the
[Hierarchical Autoregressive Transformers
paper](https://arxiv.org/abs/2501.10322v2). You can use this to implement
training and inference of HAT models.

## Installation

```bash
pip install hat-splitter
```

## Usage

```python
from hat_splitter import HATSplitter

my_hat_splitter = HATSplitter()
words: list[str] = my_hat_splitter.split("Hello, world!")
assert words == ["Hello,", " world!"]

words: list[bytes] = my_hat_splitter.split_with_limit("Hello, world!", 4)
assert words == [b'Hell', b'o,', b' wor', b'ld!']
```
