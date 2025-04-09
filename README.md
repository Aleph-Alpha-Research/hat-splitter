# hat-splitter

This is the home of the HAT splitting rule. We expose it as a Rust crate with
Python bindings so that we can use the same splitting rule in both languages.

- Rust crate: https://crates.io/crates/hat-splitter
- Python package: https://pypi.org/project/hat-splitter

## Development

See `bindings/python/README.md` for more information on the Python bindings.

### Release process

1. Update the version in `Cargo.toml`. Commit and push to `main`.
2. Tag the commit with the new version, e.g., `git tag v0.1.0`.
3. Push the tag to the remote. CI will take care of the rest.

## License

See [LICENSE](LICENSE).
