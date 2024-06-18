## Build doc locally

```sh
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features --open
```

## Publish

```sh
cargo publish --registry crates-io
```
