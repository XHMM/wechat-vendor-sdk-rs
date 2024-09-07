## Add new apis

1. after adding new api wrappers, you should write test cases under `examples/demo.rs` and have a real test to make sure you have a correct type definition

## Build doc locally

```sh
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features --open
```

## Publish

```sh
cargo publish --registry crates-io
```
