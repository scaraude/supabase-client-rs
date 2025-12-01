//! Traits for extensibility and future community crates.
//!
//! These traits define the interfaces that auth, storage, and functions
//! providers must implement. This allows the community to create their
//! own implementations while maintaining compatibility with the main client.

use crate::error::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;

// Re-export async_trait for implementors
pub use async_trait::async_trait;

/// Authentication provider trait.
///
/// Implement this trait to provide authentication functionality.
/// The community can create crates like `supabase-auth-rs` that implement this.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// The user type returned by auth operations.
    type User: DeserializeOwned + Send;
    
    /// The session type.
    type Session: DeserializeOwned + Send;

    /// Sign up with email and password.
    async fn sign_up_with_email(&self, email: &str, password: &str) -> Result<Self::Session>;

    /// Sign in with email and password.
    async fn sign_in_with_email(&self, email: &str, password: &str) -> Result<Self::Session>;

    /// Sign out the current user.
    async fn sign_out(&self) -> Result<()>;

    /// Get the current session.
    async fn get_session(&self) -> Result<Option<Self::Session>>;

    /// Get the current user.
    async fn get_user(&self) -> Result<Option<Self::User>>;

    /// Refresh the session token.
    async fn refresh_session(&self) -> Result<Self::Session>;
}

/// Storage provider trait.
///
/// Implement this trait to provide file storage functionality.
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Upload a file to a bucket.
    async fn upload(
        &self,
        bucket: &str,
        path: &str,
        data: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<String>;

    /// Download a file from a bucket.
    async fn download(&self, bucket: &str, path: &str) -> Result<Vec<u8>>;

    /// Delete a file from a bucket.
    async fn remove(&self, bucket: &str, paths: &[&str]) -> Result<()>;

    /// List files in a bucket path.
    async fn list(&self, bucket: &str, path: Option<&str>) -> Result<Vec<StorageObject>>;

    /// Get a public URL for a file.
    fn get_public_url(&self, bucket: &str, path: &str) -> String;

    /// Create a signed URL for temporary access.
    async fn create_signed_url(
        &self,
        bucket: &str,
        path: &str,
        expires_in: u64,
    ) -> Result<String>;
}

/// A storage object (file or folder).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct StorageObject {
    pub name: String,
    pub id: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub last_accessed_at: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Edge Functions provider trait.
#[async_trait]
pub trait FunctionsProvider: Send + Sync {
    /// Invoke an edge function.
    async fn invoke<T, R>(&self, function_name: &str, body: Option<T>) -> Result<R>
    where
        T: Serialize + Send + Sync,
        R: DeserializeOwned;

    /// Invoke an edge function and return raw bytes.
    async fn invoke_raw<T>(&self, function_name: &str, body: Option<T>) -> Result<Vec<u8>>
    where
        T: Serialize + Send + Sync;
}

/// Realtime provider trait.
///
/// This is a simplified interface - your `supabase-realtime-rs` can implement this.
pub trait RealtimeProvider: Send + Sync {
    /// The channel type.
    type Channel;

    /// Create or get a channel.
    fn channel(&self, name: &str) -> Self::Channel;

    /// Remove a channel.
    fn remove_channel(&self, name: &str);

    /// Get all active channels.
    fn get_channels(&self) -> Vec<String>;
}
