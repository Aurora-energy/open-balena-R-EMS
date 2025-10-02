# Development Roadmap

This roadmap decomposes the platform into manageable milestones so the team can iterate safely.

## Phase 0 – Environment Preparation
- [ ] Run `scripts/install_rust_fleet_blueprint.sh` to provision Rust toolchain, Docker Engine, and SurrealDB CLI.
- [ ] Stand up a local SurrealDB instance (`surreal start --log debug memory`) for prototyping.
- [ ] Review the architecture documents and align on priorities.

## Phase 1 – Core Control Plane
- [ ] Scaffold the `control-plane` crate with Axum-based REST endpoints and JWT authentication middleware.
- [ ] Implement SurrealDB schema migrations covering: organizations, fleets, devices, releases, VPN peers.
- [ ] Integrate command scheduling via AMQP (RabbitMQ) or Redis Streams.
- [ ] Provide OpenAPI documentation and contract tests.

## Phase 2 – Device Agent
- [ ] Build a `device-agent` crate using Tokio for async runtime and Clap for CLI configuration.
- [ ] Implement WireGuard client provisioning using the `wireguard-control` crate.
- [ ] Add Docker orchestration using the `bollard` crate to pull/start/stop containers.
- [ ] Stream heartbeat and telemetry events back to the control plane.

## Phase 3 – Secure Networking
- [ ] Deploy a WireGuard control service capable of issuing keys and generating peer configs on demand.
- [ ] Automate VPN peer lifecycle (creation, rotation, revocation) with audit trails in SurrealDB.
- [ ] Provide infrastructure as code (Terraform/Ansible) templates to bootstrap the control plane in different environments.

## Phase 4 – Observability & UX
- [ ] Integrate tracing/metrics exporters (OpenTelemetry) across control plane and device agent.
- [ ] Build a lightweight web console (could leverage Yew or Tauri) for fleet status visualization.
- [ ] Document operational runbooks and postmortem templates.

## Phase 5 – Hardening & Release
- [ ] Conduct security reviews focusing on secrets handling and VPN surface area.
- [ ] Implement automated end-to-end tests using containers simulating fleets.
- [ ] Prepare packaging artifacts (Deb/RPM) and container images for both control plane and agent.

Each phase should conclude with documentation updates in `/docs`, `/wiki`, and `/web` to keep stakeholders informed.
