# Rust Fleet Blueprint

A portable, Rust-native alternative to openBalena for securely managing fleets of Linux devices.

## Why Rust Fleet?
- **Cross-platform**: Target any Linux distribution capable of running Docker and WireGuard.
- **Security-first**: WireGuard VPN overlay with auditable provisioning flows.
- **Productive**: Async Rust stack with Axum, SurrealDB, and comprehensive observability.

## Key Features
- Control plane APIs for device enrollment, release management, and telemetry.
- Device agent capable of orchestrating Docker workloads locally.
- Extensible plugin model for CI/CD, monitoring, and secrets management.

## Roadmap Highlights
1. Scaffold Axum control plane with JWT-based authentication.
2. Implement WireGuard provisioning service and SurrealDB schema migrations.
3. Deliver an operator dashboard and automated rollout strategies.

## Learn More
- [Architecture](../rust-fleet-blueprint/architecture.md)
- [Development Roadmap](../rust-fleet-blueprint/development-roadmap.md)
- [Administrator Guide](../docs/README.md)
- [End-User Guide](../wiki/README.md)

Follow progress and contribute via GitHub issues and pull requests.
