# Rust extensions for containerd

[![CI](https://github.com/mxpv/shim-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/mxpv/shim-rs/actions/workflows/ci.yml)

A collection of Rust crates to extend containerd.

This repository contains the following crates:

| Name | Description |
| --- | --- |
| [containerd-shim-protos](crates/shim-protos) | TTRPC bindings to shim interfaces |
| [containerd-shim-logging](crates/logging) | Shim logger |
| [containerd-shim](crates/shim) | Runtime v2 shim wrapper 🚧 |
| [containerd-client-protos](crates/client-protos) | GRPC bindings to containerd APIs |

## How to build
The build process as easy as:
```bash
cargo build --release
```
