//! # supabase-client-rs
//!
//! A Rust client for [Supabase](https://supabase.com), the open-source Firebase alternative.
//!
//! This crate provides a unified interface to Supabase services by composing existing
//! community crates:
//!
//! - **Database**: Uses [`postgrest-rs`](https://crates.io/crates/postgrest) for PostgREST queries
//! - **Realtime**: Integrates with [`supabase-realtime-rs`](https://github.com/scaraude/supabase-realtime-rs)
//! - **Auth, Storage, Functions**: Extensible via traits for community implementations
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use supabase_client_rs::SupabaseClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client
//!     let client = SupabaseClient::new(
//!         "https://your-project.supabase.co",
//!         "your-anon-key"
//!     )?;
//!
//!     // Query the database
//!     let response = client
//!         .from("users")
//!         .select("id, name, email")
//!         .eq("active", "true")
//!         .execute()
//!         .await?;
//!
//!     let body = response.text().await?;
//!     println!("Users: {}", body);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Database Queries
//!
//! The client wraps `postgrest-rs` and provides a fluent API for database operations:
//!
//! ```rust,no_run
//! # use supabase_client_rs::SupabaseClient;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SupabaseClient::new("url", "key")?;
//! // Select with filters
//! let users = client
//!     .from("users")
//!     .select("*")
//!     .eq("status", "active")
//!     .order("created_at.desc")
//!     .limit(10)
//!     .execute()
//!     .await?;
//!
//! // Insert
//! let new_user = client
//!     .from("users")
//!     .insert(r#"{"name": "Alice", "email": "alice@example.com"}"#)
//!     .execute()
//!     .await?;
//!
//! // Update
//! let updated = client
//!     .from("users")
//!     .update(r#"{"status": "inactive"}"#)
//!     .eq("id", "123")
//!     .execute()
//!     .await?;
//!
//! // Delete
//! let deleted = client
//!     .from("users")
//!     .delete()
//!     .eq("id", "123")
//!     .execute()
//!     .await?;
//!
//! // RPC (stored procedures)
//! let result = client
//!     .rpc("get_user_stats", r#"{"user_id": "123"}"#)
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuration
//!
//! For advanced configuration, use `SupabaseConfig`:
//!
//! ```rust,no_run
//! use supabase_client_rs::{SupabaseClient, SupabaseConfig};
//! use std::time::Duration;
//!
//! let config = SupabaseConfig::new(
//!     "https://your-project.supabase.co",
//!     "your-anon-key"
//! )
//! .schema("custom_schema")
//! .timeout(Duration::from_secs(60))
//! .header("X-Custom-Header", "value");
//!
//! let client = SupabaseClient::with_config(config).unwrap();
//! ```
//!
//! ## Authenticated Requests
//!
//! After a user signs in, set their JWT:
//!
//! ```rust,no_run
//! # use supabase_client_rs::SupabaseClient;
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = SupabaseClient::new("url", "key")?;
//! // Get JWT from your auth flow
//! let user_jwt = "eyJhbGciOiJIUzI1NiIs...";
//!
//! // Create an authenticated client
//! let auth_client = client.with_jwt(user_jwt)?;
//!
//! // Now requests include the user's JWT
//! // RLS policies will apply based on the user
//! # Ok(())
//! # }
//! ```
//!
//! ## Extending with Community Crates
//!
//! This crate defines traits for auth, storage, and functions that community
//! crates can implement. See the [`traits`] module for details.
//!
//! ## Feature Flags
//!
//! - `rustls` (default): Use rustls for TLS
//! - `native-tls`: Use native TLS instead of rustls
//! - `realtime`: Enable Supabase Realtime support (requires `supabase-realtime-rs`)

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

mod client;
mod config;
mod error;
pub mod traits;

// Re-export main types
pub use client::SupabaseClient;
pub use config::SupabaseConfig;
pub use error::{Error, Result};

// Re-export postgrest for advanced usage
pub use postgrest;

// Re-export realtime types when feature is enabled
#[cfg(feature = "realtime")]
pub use supabase_realtime_rs;

/// Create a new Supabase client.
///
/// This is a convenience function equivalent to `SupabaseClient::new()`.
///
/// # Example
///
/// ```rust,no_run
/// use supabase_client_rs::create_client;
///
/// let client = create_client(
///     "https://your-project.supabase.co",
///     "your-anon-key"
/// ).unwrap();
/// ```
pub fn create_client(url: impl Into<String>, api_key: impl Into<String>) -> Result<SupabaseClient> {
    SupabaseClient::new(url, api_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let client = create_client("https://example.supabase.co", "test-key");
        assert!(client.is_ok());
    }

    #[test]
    fn test_create_client_empty_url() {
        let client = create_client("", "test-key");
        assert!(client.is_err());
    }

    #[test]
    fn test_create_client_empty_key() {
        let client = create_client("https://example.supabase.co", "");
        assert!(client.is_err());
    }

    #[test]
    fn test_config_urls() {
        let config = SupabaseConfig::new("https://example.supabase.co", "key");
        assert_eq!(config.rest_url(), "https://example.supabase.co/rest/v1");
        assert_eq!(config.auth_url(), "https://example.supabase.co/auth/v1");
        assert_eq!(
            config.storage_url(),
            "https://example.supabase.co/storage/v1"
        );
        assert_eq!(
            config.realtime_url(),
            "wss://example.supabase.co/realtime/v1"
        );
        assert_eq!(
            config.functions_url(),
            "https://example.supabase.co/functions/v1"
        );
    }

    #[test]
    fn test_config_trailing_slash() {
        let config = SupabaseConfig::new("https://example.supabase.co/", "key");
        assert_eq!(config.rest_url(), "https://example.supabase.co/rest/v1");
    }

    #[test]
    fn test_with_jwt() {
        let client = create_client("https://example.supabase.co", "test-key").unwrap();
        let auth_client = client.with_jwt("user-jwt");
        assert!(auth_client.is_ok());
    }

    #[test]
    fn test_config_builder() {
        let config = SupabaseConfig::new("https://example.supabase.co", "key")
            .schema("custom")
            .timeout(std::time::Duration::from_secs(60))
            .header("X-Test", "value")
            .auto_refresh_token(false);

        assert_eq!(config.schema, "custom");
        assert_eq!(config.timeout, std::time::Duration::from_secs(60));
        assert!(!config.auto_refresh_token);
    }
}
