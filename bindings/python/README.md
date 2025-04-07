# hat-splitter

These are the Python bindings for the HAT splitting rule.

## Development

The Python development environment is managed with `uv`. Install it if you
haven't already.

1. Build the crate and install it as a Python module in the `uv`-managed venv:

```bash
uv run maturin develop
```

2. Run the tests:

```bash
uv run pytest
```
