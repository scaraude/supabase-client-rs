//! Configuration types for the Supabase client.

use std::time::Duration;

/// Configuration options for the Supabase client.
#[derive(Debug, Clone)]
pub struct SupabaseConfig {
    /// The Supabase project URL (e.g., `https://xyzcompany.supabase.co`)
    pub url: String,

    /// The Supabase API key (anon key or service role key)
    pub api_key: String,

    /// Optional JWT for authenticated requests
    pub jwt: Option<String>,

    /// Custom schema (default: "public")
    pub schema: String,

    /// Request timeout
    pub timeout: Duration,

    /// Custom headers to include in all requests
    pub headers: Vec<(String, String)>,

    /// Auto-refresh token settings
    pub auto_refresh_token: bool,

    /// Persist session
    pub persist_session: bool,
}

impl SupabaseConfig {
    /// Create a new configuration with the given URL and API key.
    pub fn new(url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            api_key: api_key.into(),
            jwt: None,
            schema: "public".to_string(),
            timeout: Duration::from_secs(30),
            headers: Vec::new(),
            auto_refresh_token: true,
            persist_session: true,
        }
    }

    /// Set a custom schema.
    pub fn schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = schema.into();
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set a JWT for authenticated requests.
    pub fn jwt(mut self, jwt: impl Into<String>) -> Self {
        self.jwt = Some(jwt.into());
        self
    }

    /// Add a custom header.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Disable auto-refresh token.
    pub fn auto_refresh_token(mut self, enabled: bool) -> Self {
        self.auto_refresh_token = enabled;
        self
    }

    /// Disable session persistence.
    pub fn persist_session(mut self, enabled: bool) -> Self {
        self.persist_session = enabled;
        self
    }

    /// Get the REST API URL.
    pub fn rest_url(&self) -> String {
        format!("{}/rest/v1", self.url.trim_end_matches('/'))
    }

    /// Get the Auth API URL.
    pub fn auth_url(&self) -> String {
        format!("{}/auth/v1", self.url.trim_end_matches('/'))
    }

    /// Get the Storage API URL.
    pub fn storage_url(&self) -> String {
        format!("{}/storage/v1", self.url.trim_end_matches('/'))
    }

    /// Get the Realtime URL.
    pub fn realtime_url(&self) -> String {
        let base = self.url.trim_end_matches('/');
        let ws_url = base
            .replace("https://", "wss://")
            .replace("http://", "ws://");
        format!("{}/realtime/v1", ws_url)
    }

    /// Get the Functions URL.
    pub fn functions_url(&self) -> String {
        format!("{}/functions/v1", self.url.trim_end_matches('/'))
    }
}
