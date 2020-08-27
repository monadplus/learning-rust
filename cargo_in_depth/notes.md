## Customizing Builds with Release Profiles

Cargo has two main profiles (with default options):
- dev: `$ cargo build`
- release: `$ cargo build --release`

You can add a new profile at `Cargo.toml`.

## Publishing a Crate to Crates.io

`$ cargo doc`, and open at _target/doc_ or `$ cargo doc --open`

`$ cargo test` will also run the test in your documentation.
