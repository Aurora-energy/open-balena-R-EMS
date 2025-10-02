# End-User Guide

This guide explains how operators interact with the Rust Fleet Blueprint once the control plane and device agents are deployed.

## Concepts
- **Fleet** – A logical grouping of devices that share deployment policies.
- **Release** – A versioned container workload stored in a registry and assigned to a fleet.
- **Device Agent** – Rust binary running on each Linux device to maintain VPN connectivity and execute jobs.

## Typical Workflow
1. **Register a Device**
   - Obtain a bootstrap token from an administrator.
   - Run the device agent with `--enroll` to generate a WireGuard key pair and register with the control plane.
   - Confirm the device appears in the dashboard.
2. **Deploy an Update**
   - Select a fleet and attach a release (OCI image reference).
   - The control plane schedules the job; device agents pull and run the container via Docker.
3. **Monitor Health**
   - Use the dashboard or CLI to review heartbeat timestamps, container status, and telemetry metrics.
   - Devices report logs over the VPN, allowing operators to debug remotely.

## CLI Quick Start
Until the full UX is complete, operators can interact with the prototype CLI:
```bash
cargo run --bin rust-fleet-blueprint -- enroll-device \
  --device-name kiosk-001 \
  --public-key "device-public-key"
```

## Troubleshooting
- **Device offline**: Check VPN connectivity using `wg show` on both ends.
- **Deployment stuck**: Verify Docker daemon is healthy (`systemctl status docker`) and the device has enough disk space.
- **Authentication failures**: Ensure the bootstrap token has not expired and that the device clock is synchronized via NTP.

Feedback from early operators should be documented here to improve future iterations of the UX.
