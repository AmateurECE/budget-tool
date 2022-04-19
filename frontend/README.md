# Building Frontend

```
$ wasm-pack build --target web
$ rollup main.js --format=iife -o pkg/bundle.js
```

# Testing

```
$ podman-compose up &
$ cargo run &
$ cd frontend && RUST_LOG=tower_http=trace dev-prox
```
