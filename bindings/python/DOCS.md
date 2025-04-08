# hat-splitter

The `hat-splitter` package implements the HAT splitting rule.

This crate is a work in progress. More information and documentation will
follow.

## Installation

```bash
pip install hat-splitter
```

## Usage

```python
from hat_splitter import HATSplitter

my_hat_splitter = HATSplitter()
split_text: str = my_hat_splitter.split("This is a test sentence.")
split_text_bytes: list[bytes] = my_hat_splitter.split_bytes("This is a test sentence.")
```
