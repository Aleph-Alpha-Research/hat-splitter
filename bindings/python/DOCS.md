# hat-splitter

The `hat-splitter` package implements the HAT splitting rule. If you don't know
what that is, you probably won't find this package useful.

## Installation

```bash
pip install hat-splitter
```

## Usage

```python
from hat_splitter import HATSplitter

my_hat_splitter = HATSplitter()
split_text: list[str] = my_hat_splitter.split("Hello, world!")
assert split_text == ["Hello,", " world!"]

split_text: list[bytes] = my_hat_splitter.split_with_limit("Hello, world!", 4)
assert split_text == [b'Hell', b'o,', b' wor', b'ld!']
```
