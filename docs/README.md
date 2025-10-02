# Administrator Setup Guide

This document helps platform administrators reproduce the Rust Fleet Blueprint in a laboratory environment. The focus is on preparing infrastructure analogous to openBalena while remaining portable across Linux distributions.

## 1. Prerequisites
- 64-bit Linux host (bare metal, VM, or cloud instance) with root access.
- Outbound internet connectivity to fetch crates, Docker images, and SurrealDB binaries.
- Ability to install WireGuard kernel modules (`wg`, `wg-quick`).

## 2. Toolchain Provisioning
1. Clone this repository.
2. Execute `./scripts/install_rust_fleet_blueprint.sh`.
   - The script installs the Rust toolchain via `rustup`, Docker Engine (if available through APT), and the SurrealDB CLI.
   - A log file is generated under `scripts/logs/install_rust_fleet_blueprint.log` for auditing.
3. Verify the environment:
   ```bash
   rustc --version
   cargo --version
   docker --version
   surreal version
   ```

## 3. SurrealDB Bootstrap
1. Start SurrealDB in memory mode for quick tests:
   ```bash
   surreal start --log info memory
   ```
2. For persistent storage, deploy SurrealDB with RocksDB backing:
   ```bash
   surreal start --log info file://$HOME/.local/share/surrealdb
   ```
3. Create a dedicated namespace/database for the fleet platform:
   ```sql
   USE NS fleet DB rust_fleet;
   ```

## 4. WireGuard Authority Prototype
1. Ensure WireGuard tools are installed (`sudo apt install wireguard-tools`).
2. Run the sample CLI to issue a configuration (requires SurrealDB to be running):
   ```bash
   cargo run --bin rust-fleet-blueprint -- issue-vpn-config --device-id demo-device
   ```
3. Inspect the emitted configuration and apply it on a test device using `wg-quick`.

## 5. Docker Runtime Validation
1. Confirm Docker daemon is active: `sudo systemctl status docker`.
2. Pull a sample image to validate network access: `docker pull hello-world`.
3. Review `rust-fleet-blueprint/src/orchestrator.rs` for guidance on integrating with message queues before orchestrating container deployments.

## 6. Next Steps
- Use `rust-fleet-blueprint/src/main.rs` as a template for developing Axum-based APIs.
- Extend the SurrealDB schema to support organizations, releases, and audit logs.
- Integrate WireGuard provisioning with actual `wireguard-control` commands executed under root privileges.

Whenever new functionality is added, update this guide to document any additional system dependencies or configuration steps.
