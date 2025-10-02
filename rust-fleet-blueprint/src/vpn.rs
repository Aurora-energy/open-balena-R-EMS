//! WireGuard authority mock.
//!
//! openBalena provisions a VPN so operators can securely access devices. This
//! module sketches how a Rust service could manage WireGuard peers using the
//! `wireguard-control` crate. Because provisioning requires elevated privileges
//! and kernel support, the functions shown here only emulate the configuration
//! flow while explaining the required steps.

use std::fmt::Write as _;

use thiserror::Error;
use tracing::instrument;

/// Simplified configuration object returned to device agents. In a production
/// implementation we would generate a full `wg-quick` compatible config file.
#[derive(Debug, Clone)]
pub struct WireGuardConfig {
    /// Name of the WireGuard interface that the device should create.
    pub interface: String,
    /// Base64-encoded private key issued to the device.
    pub private_key: String,
    /// Endpoint of the control plane WireGuard server.
    pub endpoint: String,
    /// Allowed IP ranges for routed traffic.
    pub allowed_ips: Vec<String>,
}

/// Abstraction responsible for managing WireGuard peers.
#[derive(Default)]
pub struct WireGuardAuthority;

impl WireGuardAuthority {
    /// Issues a configuration for the requested device. The method demonstrates
    /// how we would normally allocate keys, register the peer, and format the
    /// resulting configuration file.
    #[instrument(skip(self))]
    pub async fn issue_peer_config(
        &self,
        device_id: &str,
    ) -> Result<String, WireGuardError> {
        // In production we would call into `wireguard-control` to create a peer
        // on the server interface and persist the peer metadata in SurrealDB.
        // For now we simulate key generation and config assembly for clarity.
        let private_key = self.generate_private_key(device_id).await?;
        let config = WireGuardConfig {
            interface: format!("wg-{}", device_id),
            private_key,
            endpoint: "vpn.rust-fleet.local:51820".into(),
            allowed_ips: vec!["0.0.0.0/0".into(), "::/0".into()],
        };

        Ok(format_config(&config))
    }

    /// Placeholder key generation. Replace with actual `wireguard-control`
    /// integration or a call to a secure key management service.
    async fn generate_private_key(&self, device_id: &str) -> Result<String, WireGuardError> {
        // Deterministic fake key to keep the example reproducible.
        let digest = blake3::hash(device_id.as_bytes());
        Ok(base64::encode(digest.as_bytes()))
    }
}

/// Pretty-prints the configuration so it can be saved as a `.conf` file on the
/// device. Having a centralized function makes it easier to later support other
/// formats (JSON API responses, YAML, etc.).
fn format_config(config: &WireGuardConfig) -> String {
    let mut buffer = String::new();
    let _ = writeln!(&mut buffer, "[Interface]");
    let _ = writeln!(&mut buffer, "PrivateKey = {}", config.private_key);
    let _ = writeln!(&mut buffer, "Address = 10.42.0.1/24");
    let _ = writeln!(&mut buffer);
    let _ = writeln!(&mut buffer, "[Peer]");
    let _ = writeln!(&mut buffer, "Endpoint = {}", config.endpoint);
    let _ = writeln!(&mut buffer, "AllowedIPs = {}", config.allowed_ips.join(","));
    buffer
}

/// Errors that could occur while provisioning a peer.
#[derive(Debug, Error)]
pub enum WireGuardError {
    /// Represents failures when generating key material.
    #[error("failed to generate wireguard key: {0}")]
    KeyGeneration(String),
}
