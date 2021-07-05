# bouncer

This repo is a toy admission controller in Rust. This controller checks if a liveness/readiness probe is present on a checked namespace and rejects the manifest if both are missing.
