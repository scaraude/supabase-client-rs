//! The main Supabase client.

use crate::config::SupabaseConfig;
use crate::error::{Error, Result};
use postgrest::Postgrest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION};

#[cfg(feature = "realtime")]
use supabase_realtime_rs::{RealtimeClient, RealtimeClientOptions};

/// The main Supabase client.
///
/// This client provides access to all Supabase services:
/// - Database queries via PostgREST (`.from()`)
/// - Realtime subscriptions (`.realtime()`) - requires `realtime` feature
/// - Authentication (`.auth()`) - when community crate is available
/// - Storage (`.storage()`) - when community crate is available
/// - Edge Functions (`.functions()`) - when community crate is available
///
/// # Example
///
/// ```rust,no_run
/// use supabase_client_rs::SupabaseClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SupabaseClient::new(
///         "https://your-project.supabase.co",
///         "your-anon-key"
///     )?;
///
///     // Query the database
///     let response = client
///         .from("users")
///         .select("*")
///         .execute()
///         .await?;
///
///     println!("{}", response.text().await?);
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct SupabaseClient {
    config: SupabaseConfig,
    http: reqwest::Client,
    postgrest: Postgrest,
    #[cfg(feature = "realtime")]
    realtime: std::sync::Arc<RealtimeClient>,
}

impl SupabaseClient {
    /// Create a new Supabase client with the given URL and API key.
    ///
    /// # Arguments
    ///
    /// * `url` - The Supabase project URL (e.g., `https://xyzcompany.supabase.co`)
    /// * `api_key` - The Supabase API key (anon or service role)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use supabase_client_rs::SupabaseClient;
    ///
    /// let client = SupabaseClient::new(
    ///     "https://your-project.supabase.co",
    ///     "your-anon-key"
    /// ).unwrap();
    /// ```
    pub fn new(url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let config = SupabaseConfig::new(url, api_key);
        Self::with_config(config)
    }

    /// Create a new Supabase client with custom configuration.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use supabase_client_rs::{SupabaseClient, SupabaseConfig};
    /// use std::time::Duration;
    ///
    /// let config = SupabaseConfig::new(
    ///     "https://your-project.supabase.co",
    ///     "your-anon-key"
    /// )
    /// .schema("custom_schema")
    /// .timeout(Duration::from_secs(60));
    ///
    /// let client = SupabaseClient::with_config(config).unwrap();
    /// ```
    pub fn with_config(config: SupabaseConfig) -> Result<Self> {
        if config.url.is_empty() {
            return Err(Error::config("URL is required"));
        }
        if config.api_key.is_empty() {
            return Err(Error::config("API key is required"));
        }

        // Build default headers
        let mut headers = HeaderMap::new();
        headers.insert(
            "apikey",
            HeaderValue::from_str(&config.api_key).map_err(|e| Error::config(e.to_string()))?,
        );

        // Add Authorization header
        let auth_value = if let Some(ref jwt) = config.jwt {
            format!("Bearer {}", jwt)
        } else {
            format!("Bearer {}", config.api_key)
        };
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).map_err(|e| Error::config(e.to_string()))?,
        );

        // Add custom headers
        for (key, value) in &config.headers {
            let name = HeaderName::try_from(key.as_str())
                .map_err(|e| Error::config(format!("invalid header name: {}", e)))?;
            let val = HeaderValue::from_str(value)
                .map_err(|e| Error::config(format!("invalid header value: {}", e)))?;
            headers.insert(name, val);
        }

        // Build HTTP client
        let http = reqwest::Client::builder()
            .default_headers(headers.clone())
            .timeout(config.timeout)
            .build()?;

        // Build PostgREST client
        let postgrest = Postgrest::new(config.rest_url())
            .insert_header("apikey", &config.api_key)
            .insert_header("Authorization", &auth_value);

        // Build Realtime client if feature is enabled
        #[cfg(feature = "realtime")]
        let realtime = {
            let realtime_client = RealtimeClient::new(
                &config.realtime_url(),
                RealtimeClientOptions {
                    api_key: config.api_key.clone(),
                    ..Default::default()
                },
            )?;
            std::sync::Arc::new(realtime_client)
        };

        Ok(Self {
            config: config.clone(),
            http,
            postgrest,
            #[cfg(feature = "realtime")]
            realtime,
        })
    }

    /// Create a query builder for the given table.
    ///
    /// This is the main entry point for database operations using PostgREST.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_client_rs::SupabaseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SupabaseClient::new("url", "key")?;
    /// // Select all users
    /// let users = client.from("users").select("*").execute().await?;
    ///
    /// // Insert a new user
    /// let new_user = client
    ///     .from("users")
    ///     .insert(r#"{"name": "Alice", "email": "alice@example.com"}"#)
    ///     .execute()
    ///     .await?;
    ///
    /// // Update with filters
    /// let updated = client
    ///     .from("users")
    ///     .update(r#"{"status": "active"}"#)
    ///     .eq("id", "123")
    ///     .execute()
    ///     .await?;
    ///
    /// // Delete with filters
    /// let deleted = client
    ///     .from("users")
    ///     .delete()
    ///     .eq("status", "inactive")
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from(&self, table: &str) -> postgrest::Builder {
        self.postgrest.from(table)
    }

    /// Execute a stored procedure (RPC).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_client_rs::SupabaseClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SupabaseClient::new("url", "key")?;
    /// let result = client
    ///     .rpc("my_function", r#"{"param1": "value1"}"#)
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn rpc(&self, function: &str, params: &str) -> postgrest::Builder {
        self.postgrest.rpc(function, params)
    }

    /// Get the configuration.
    pub fn config(&self) -> &SupabaseConfig {
        &self.config
    }

    /// Get the underlying HTTP client.
    ///
    /// Useful for making custom requests to Supabase APIs.
    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    /// Get the PostgREST client.
    ///
    /// Use this if you need direct access to the PostgREST client.
    pub fn postgrest(&self) -> &Postgrest {
        &self.postgrest
    }

    /// Set a JWT for authenticated requests.
    ///
    /// This creates a new client with the updated JWT.
    /// Use this after a user signs in to make authenticated requests.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_client_rs::SupabaseClient;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = SupabaseClient::new("url", "key")?;
    /// let jwt = "user-jwt-token";
    /// let authenticated_client = client.with_jwt(jwt)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_jwt(&self, jwt: impl Into<String>) -> Result<Self> {
        let mut new_config: SupabaseConfig = self.config.clone();
        new_config.jwt = Some(jwt.into());
        Self::with_config(new_config)
    }

    /*
    // =========================================================================
    // Future: Auth, Storage, Functions, Realtime
    // These will be enabled when community crates are available
    // =========================================================================
    /// Access the Auth client.
    ///
    /// **Note:** This requires an auth provider to be set up.
    /// See the `supabase-auth-rs` crate (when available).
    #[cfg(feature = "auth")]
    pub fn auth(&self) -> &dyn crate::traits::AuthProvider {
        todo!("Auth provider not yet implemented - contribute at supabase-auth-rs!")
    }

    /// Access the Storage client.
    ///
    /// **Note:** This requires a storage provider to be set up.
    /// See the `supabase-storage-rs` crate (when available).
    #[cfg(feature = "storage")]
    pub fn storage(&self) -> &dyn crate::traits::StorageProvider {
        todo!("Storage provider not yet implemented - contribute at supabase-storage-rs!")
    }

    /// Access the Functions client.
    ///
    /// **Note:** This requires a functions provider to be set up.
    /// See the `supabase-functions-rs` crate (when available).
    #[cfg(feature = "functions")]
    pub fn functions(&self) -> &dyn crate::traits::FunctionsProvider {
        todo!("Functions provider not yet implemented - contribute at supabase-functions-rs!")
    }
    */

    // =========================================================================
    // Realtime - Integration with supabase-realtime-rs
    // =========================================================================

    /// Get the Realtime client.
    ///
    /// Requires the `realtime` feature to be enabled.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "realtime")]
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use supabase_client_rs::SupabaseClient;
    /// # use supabase_realtime_rs::{ChannelEvent, RealtimeChannelOptions};
    /// # let client = SupabaseClient::new("url", "key")?;
    /// // Get the realtime client
    /// let realtime = client.realtime();
    ///
    /// // Connect to realtime
    /// realtime.connect().await?;
    ///
    /// // Create a channel
    /// let channel = realtime.channel("room:lobby", RealtimeChannelOptions::default()).await;
    /// let mut rx = channel.on(ChannelEvent::broadcast("message")).await;
    /// channel.subscribe().await?;
    ///
    /// // Listen for messages
    /// tokio::spawn(async move {
    ///     while let Some(msg) = rx.recv().await {
    ///         println!("Received: {:?}", msg);
    ///     }
    /// });
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "realtime")]
    pub fn realtime(&self) -> &RealtimeClient {
        &self.realtime
    }

    /// Get the Realtime WebSocket URL.
    ///
    /// Use this to initialize your own `supabase-realtime-rs` client if needed.
    pub fn realtime_url(&self) -> String {
        self.config.realtime_url()
    }
}

impl std::fmt::Debug for SupabaseClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SupabaseClient")
            .field("url", &self.config.url)
            .field("schema", &self.config.schema)
            .finish_non_exhaustive()
    }
}
