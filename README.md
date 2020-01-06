# Wasm Conway

A repository to go along with my presentation for Rust Wrocław Meetup.

## Rust

The rust package has 2 features to take note of:

1. `wasm` enabled by default, use with `wasm-pack build` to build the wasm binary and JS bindings in `pkg`.
2. `python_wasmtime` exposes manually written low-level functions for use in Python/Wasmtime

## Web

The `www` directory contains a package generated from [`create-wasm-app`](https://github.com/rustwasm/create-wasm-app) template.

## Python
Build and move the wasm file with `prepare_python_wasmtime.sh`.

Run `python/main.py`
