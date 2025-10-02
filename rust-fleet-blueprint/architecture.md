# Architecture Overview

The proposed platform mirrors the capabilities of openBalena but reimagines the stack for portable deployment on any Linux distribution. The architecture is divided into the following logical layers:

## 1. Control Plane

| Component | Responsibility | Rust Crates / Services |
| --- | --- | --- |
| **API Gateway** | Exposes REST/gRPC endpoints for device enrollment, job submission, and fleet insights. Handles authentication/authorization using JWT or OIDC. | [`axum`](https://crates.io/crates/axum), [`tower`](https://crates.io/crates/tower), [`jsonwebtoken`](https://crates.io/crates/jsonwebtoken) |
| **Command Scheduler** | Queues and dispatches device actions (deploy container, update configuration). Supports retry policies and observability. | [`lapin`](https://crates.io/crates/lapin) for AMQP, [`tracing`](https://crates.io/crates/tracing) |
| **SurrealDB Metadata Store** | Persists devices, releases, VPN identities, and audit trails. | [`surrealdb`](https://crates.io/crates/surrealdb) |
| **VPN Authority** | Issues and rotates WireGuard keys; manages peer configuration state. | [`wireguard-control`](https://crates.io/crates/wireguard-control) |
| **Container Registry Proxy** | Caches OCI images locally so devices can pull updates even in constrained networks. | [`oci-distribution`](https://crates.io/crates/oci-distribution) |

## 2. Device Plane

| Component | Responsibility | Rust Crates / Services |
| --- | --- | --- |
| **Device Agent** | Runs on each managed host, maintaining VPN connectivity, performing health checks, and executing commands from the control plane. | [`tokio`](https://crates.io/crates/tokio), [`reqwest`](https://crates.io/crates/reqwest), [`serde`](https://crates.io/crates/serde) |
| **Docker Runtime Bridge** | Interacts with the local Docker Engine to run workloads inside containers. | [`bollard`](https://crates.io/crates/bollard) |
| **Secure Tunnel** | WireGuard tunnel connecting the device to control plane services. | [`wireguard-control`](https://crates.io/crates/wireguard-control) |
| **Telemetry Collector** | Streams metrics and logs back to the control plane for observability. | [`opentelemetry`](https://crates.io/crates/opentelemetry), [`tracing`](https://crates.io/crates/tracing) |

## 3. Networking

- **WireGuard mesh** ensures all device-to-control plane traffic traverses an encrypted channel.
- **MQTT/AMQP message bus** distributes low-latency commands and telemetry.
- **HTTPS REST API** provides a universal integration interface for administrative tooling.

## 4. Security Considerations

1. **Mutual authentication**: Devices enroll via short-lived bootstrap tokens exchanged for long-term WireGuard credentials.
2. **Least privilege**: Device agents receive scoped JWT claims limiting accessible APIs.
3. **Auditability**: All actions recorded in SurrealDB with tamper-evident timestamps.
4. **Secret storage**: Optionally integrate with HashiCorp Vault (via [`vault`](https://crates.io/crates/vault)) when available.

## Data Flow Summary

1. Operator registers a device via the API Gateway.
2. SurrealDB stores the device record and issues a provisioning job to the scheduler.
3. Scheduler notifies the Device Agent through the message bus once the device establishes VPN connectivity.
4. The Device Agent pulls the desired release metadata, fetches containers from the registry proxy, and orchestrates Docker lifecycle events.
5. Telemetry flows back to the control plane for monitoring and alerting.

## Deployment Topologies

- **Single-node lab**: Run all services via Docker Compose for evaluation.
- **Hybrid cloud**: Control plane in a public cloud, devices distributed across customer sites.
- **Air-gapped**: Deploy control plane within a private data center; devices connect over dedicated VPN.

Future documentation will expand each component with configuration references and operational playbooks.
