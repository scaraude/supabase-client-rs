# supabase-rs

A Rust client for [Supabase](https://supabase.com), the open-source Firebase alternative.

[![Crates.io](https://img.shields.io/crates/v/supabase-rs.svg)](https://crates.io/crates/supabase-rs)
[![Documentation](https://docs.rs/supabase-rs/badge.svg)](https://docs.rs/supabase-rs)
[![License](https://img.shields.io/crates/l/supabase-rs.svg)](LICENSE)

## Overview

This crate provides a unified interface to Supabase services by composing existing community crates:

| Service | Status | Crate |
|---------|--------|-------|
| **Database (PostgREST)** | ✅ Ready | [`postgrest-rs`](https://crates.io/crates/postgrest) |
| **Realtime** | 🔧 Integration ready | [`supabase-realtime-rs`](https://github.com/scaraude/supabase-realtime-rs) |
| **Auth** | 📦 Trait defined | Community: TBD |
| **Storage** | 📦 Trait defined | Community: TBD |
| **Edge Functions** | 📦 Trait defined | Community: TBD |

## Installation

```toml
[dependencies]
supabase-rs = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use supabase_rs::create_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client(
        "https://your-project.supabase.co",
        "your-anon-key"
    )?;

    // Query the database
    let response = client
        .from("users")
        .select("id, name, email")
        .eq("active", "true")
        .limit(10)
        .execute()
        .await?;

    let users: Vec<serde_json::Value> = response.json().await?;
    println!("{:?}", users);

    Ok(())
}
```

## Database Queries

The client wraps `postgrest-rs` for all database operations:

```rust
// Select with filters
let users = client
    .from("users")
    .select("*")
    .eq("status", "active")
    .order("created_at.desc")
    .limit(10)
    .execute()
    .await?;

// Insert
let new_user = client
    .from("users")
    .insert(r#"{"name": "Alice", "email": "alice@example.com"}"#)
    .execute()
    .await?;

// Update
let updated = client
    .from("users")
    .update(r#"{"status": "inactive"}"#)
    .eq("id", "123")
    .execute()
    .await?;

// Delete
let deleted = client
    .from("users")
    .delete()
    .eq("id", "123")
    .execute()
    .await?;

// RPC (stored procedures)
let result = client
    .rpc("get_user_stats", r#"{"user_id": "123"}"#)
    .execute()
    .await?;
```

## Configuration

```rust
use supabase_rs::{SupabaseClient, SupabaseConfig};
use std::time::Duration;

let config = SupabaseConfig::new(
    "https://your-project.supabase.co",
    "your-anon-key"
)
.schema("custom_schema")
.timeout(Duration::from_secs(60))
.header("X-Custom-Header", "value");

let client = SupabaseClient::with_config(config)?;
```

## Authenticated Requests

After a user signs in, set their JWT for Row Level Security:

```rust
// Get JWT from your auth flow
let user_jwt = "eyJhbGciOiJIUzI1NiIs...";

// Create an authenticated client
let auth_client = client.with_jwt(user_jwt)?;

// Requests now include the user's JWT
// RLS policies will apply based on the user
```

## Realtime Integration

Get the WebSocket URL for use with `supabase-realtime-rs`:

```rust
let realtime_url = client.realtime_url();
// Use with your realtime client
```

## Contributing

Contributions are welcome! Areas that need work:

- [ ] **Auth client** - Implement `AuthProvider` trait
- [ ] **Storage client** - Implement `StorageProvider` trait  
- [ ] **Functions client** - Implement `FunctionsProvider` trait
- [ ] **Better Realtime integration** - Tighter coupling with `supabase-realtime-rs`

See the `traits` module for the interfaces to implement.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
