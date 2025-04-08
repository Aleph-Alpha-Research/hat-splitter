# hat-splitter

This is the home of the HAT splitting rule. We expose it as a Rust crate with
Python bindings so that the same splitting rule can be used in both languages.

Rust crate: https://crates.io/crates/hat-splitter
Python package: https://pypi.org/project/hat-splitter

This project is WIP. More information and documentation to follow.

## The plan

We've found that HAT models are very sensitive to their splitting rule. As a
result, the splitting rule implemented here must exactly match the behaviour of
the splitter we're currently using.

1. Create a simple placeholder text splitting implementation (e.g., just split
   on whitespace).
2. Set up Python bindings with PyO3.
3. Add Scaling as a Python dev dep and test the Python bindings against the
   existing splitting rule. Tests will fail.
4. Implement the HAT splitting rule in Rust and make tests green.

Once these basics are in place, we can start thinking about packaging and
publishing.

## Development

### Release process

1. Update the version in `Cargo.toml`. Commit and push to `main`.
2. Tag the commit with the new version, e.g., `git tag v0.1.0`.
3. Push the tag to the remote. CI will take care of the rest.
