# hat-splitter

This is the home of the HAT splitting rule. We expose it as a Rust crate with
Python bindings so that (1) we can use the same splitting rule in both
languages; and (2) we improve performance for inference.

- Rust crate: https://crates.io/crates/hat-splitter
- Python package: https://pypi.org/project/hat-splitter

## Performance

The following pytest benchmark result demonstrates the performance advantage of
this Rust implementation over the previous pure Python implementation.

```
-------------------------------------------------------
Name (time in ms)                         Mean         
-------------------------------------------------------
test_benchmark_hat_splitter            41.3459 (1.0)   
test_benchmark_scaling_splitter     2,415.7853 (58.43) 
-------------------------------------------------------
```

## Development

See [the Python bindings README.md](bindings/python/README.md) for development
instructions for the Python bindings.

### Release process

1. Update the version in `Cargo.toml`. Commit and push to `main`.
2. Tag the commit with the new version, e.g., `git tag v0.1.0`.
3. Push the tag to the remote. CI will take care of the rest.

## License

See [LICENSE](LICENSE).
