//! Database access layer for SurrealDB.
//!
//! This module encapsulates connection handling and demonstrates how a thin
//! repository abstraction could keep persistence concerns isolated from the
//! rest of the application. Real implementations would include error handling
//! for network partitions, migrations, and structured queries. Here we focus on
//! readability and education.

use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

/// Represents the database connection and provides helper methods for common
/// operations. The goal is to keep business logic in higher-level modules while
/// this type ensures consistent access patterns.
pub struct Database {
    client: Surreal<Ws>,
}

impl Database {
    /// Establishes a WebSocket connection to a SurrealDB server and selects the
    /// namespace/database pair. In production, credentials should be sourced
    /// from environment variables or a secrets manager.
    pub async fn connect(endpoint: &str, database: &str) -> anyhow::Result<Self> {
        let client = Surreal::new::<Ws>(endpoint).await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("fleet").use_db(database).await?;

        Ok(Self { client })
    }

    /// Inserts a new device record. The simplified schema demonstrates how
    /// SurrealDB's SQL-like language lets us create and return structured data
    /// with minimal boilerplate.
    pub async fn register_device(
        &self,
        device_name: &str,
        public_key: &str,
    ) -> anyhow::Result<DeviceRecord> {
        let sql = "CREATE device CONTENT { name: $name, public_key: $public_key }";
        let mut response = self
            .client
            .query(sql)
            .bind(("name", device_name))
            .bind(("public_key", public_key))
            .await?;

        let record: Option<DeviceRecord> = response.take(0)?;
        record.ok_or_else(|| anyhow::anyhow!("device creation returned no record"))
    }
}

/// Minimal device record structure that mirrors the SurrealDB schema used above.
/// In a real system this would be generated from shared schema definitions to
/// guarantee parity across services.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DeviceRecord {
    /// Unique identifier assigned by SurrealDB (e.g., `device:ul8q0w2sk73m`).
    pub id: Thing,
    /// Friendly name supplied during registration.
    pub name: String,
    /// WireGuard public key captured during bootstrap.
    pub public_key: String,
}
