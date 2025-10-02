# Rust Fleet Blueprint

This blueprint outlines a Rust-based remote device management platform inspired by openBalena's fleet management capabilities. The goal is to orchestrate secure provisioning, monitoring, and lifecycle management for heterogeneous Linux devices through:

- **Rust core services** that coordinate device onboarding, health reporting, and job execution.
- **SurrealDB** as the cloud-native database for fleet metadata and operational state.
- **Containerized workloads** executed locally via Docker Engine on each device.
- **WireGuard-based VPN overlay** to provide secure remote access and telemetry tunnels regardless of the underlying operating system.

## Repository Layout

The blueprint is intentionally documentation-heavy to guide future implementation work:

- `architecture.md` – platform components, communication flows, and security boundaries.
- `development-roadmap.md` – phased milestones that decompose the delivery into incremental steps.
- `src/` – reference Rust code snippets demonstrating critical patterns (database connectivity, VPN orchestration, Docker interactions).
- `docs/`, `wiki/`, `web/` – higher-level documentation collections maintained at the repository root.

## Getting Started

1. Read `architecture.md` to understand the high-level system design.
2. Follow `development-roadmap.md` to execute the project incrementally.
3. Explore the reference code in `src/` to familiarize yourself with recommended libraries and patterns.
4. Execute `scripts/install_rust_fleet_blueprint.sh` to provision local tooling (Rust, Docker, SurrealDB CLI) needed for experimentation.

The remainder of this folder expands on each step.
