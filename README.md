# supabase-client-rs

A unified Rust client for [Supabase](https://supabase.com), the open-source Firebase alternative.

[![Crates.io](https://img.shields.io/crates/v/supabase-client-rs.svg)](https://crates.io/crates/supabase-client-rs)
[![Documentation](https://docs.rs/supabase-client-rs/badge.svg)](https://docs.rs/supabase-client-rs)
[![License](https://img.shields.io/crates/l/supabase-client-rs.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)

## Overview

This crate provides a unified interface to Supabase services by composing existing community crates:

| Service                  | Status           | Crate                                                                      |
| ------------------------ | ---------------- | -------------------------------------------------------------------------- |
| **Database (PostgREST)** | ✅ Ready         | [`postgrest-rs`](https://crates.io/crates/postgrest)                       |
| **Realtime**             | ✅ Ready         | [`supabase-realtime-rs`](https://github.com/scaraude/supabase-realtime-rs) |
| **Auth**                 | 📦 Trait defined | Community: TBD                                                             |
| **Storage**              | 📦 Trait defined | Community: TBD                                                             |
| **Edge Functions**       | 📦 Trait defined | Community: TBD                                                             |

## Installation

```bash
cargo add supabase-client-rs
```

## Quick Start

```rust
use supabase_client_rs::create_client;

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
use supabase_client_rs::{SupabaseClient, SupabaseConfig};
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

Enable the `realtime` feature to use Supabase Realtime:

```bash
cargo add supabase-client-rs --features "realtime"
```

Then use the realtime client:

```rust
use supabase_realtime_rs::{ChannelEvent, RealtimeChannelOptions};

// Get the realtime client
let realtime = client.realtime();

// Connect to realtime
realtime.connect().await?;

// Subscribe to a channel
let channel = realtime.channel("room:lobby", RealtimeChannelOptions::default()).await;
let mut rx = channel.on(ChannelEvent::broadcast("message")).await;
channel.subscribe().await?;

// Send a message
channel.send(
    ChannelEvent::broadcast("message"),
    serde_json::json!({"text": "Hello from Rust!"})
).await?;

// Listen for messages
tokio::spawn(async move {
    while let Some(msg) = rx.recv().await {
        println!("Received: {:?}", msg);
    }
});
```

See [`examples/realtime.rs`](examples/realtime.rs) for a complete example including presence tracking and database changes.

## Features

- **Database Operations**: Full PostgREST integration with select, insert, update, delete, and RPC support
- **Realtime**: Subscribe to database changes, broadcast messages, and presence tracking
- **Type-safe**: Leverage Rust's type system for reliable code
- **Async/Await**: Built on Tokio for efficient async operations
- **Flexible Configuration**: Custom headers, schemas, and timeouts
- **JWT Authentication**: Row Level Security support with JWT tokens

## Real-World Demo

Check out realtime-chat-demo - a full-featured chat application built with this library, demonstrating broadcast messaging, presence tracking, and WebSocket connection management.

## Contributing

Contributions are welcome! Areas that need work:

- [x] **Realtime** - ✅ Integrated with `supabase-realtime-rs`
- [ ] **Auth client** - Implement `AuthProvider` trait
- [ ] **Storage client** - Implement `StorageProvider` trait
- [ ] **Functions client** - Implement `FunctionsProvider` trait

See the `traits` module for the interfaces to implement.

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Acknowledgments

This project builds on the excellent work of:

- [postgrest-rs](https://github.com/supabase-community/postgrest-rs) - PostgREST client
- [supabase-realtime-rs](https://github.com/scaraude/supabase-realtime-rs) - Realtime client
