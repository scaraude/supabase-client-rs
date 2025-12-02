//! Example demonstrating Supabase Realtime integration.
//!
//! This example shows how to:
//! - Connect to Supabase Realtime
//! - Subscribe to a channel
//! - Send and receive broadcast messages
//! - Track presence
//!
//! To run this example:
//! 1. Copy .env.example to .env
//! 2. Fill in your Supabase credentials:
//!    - SUPABASE_URL=https://your-project.supabase.co
//!    - SUPABASE_API_KEY=your-anon-key
//! 3. cargo run --example realtime --features realtime
//!
//! Note: If you see "NotConnected" errors during cleanup, this is usually
//! because the WebSocket connection has a timeout or was disconnected.
//! This is normal for idle connections and the example handles it gracefully.

use supabase_rs::SupabaseClient;

#[cfg(feature = "realtime")]
use supabase_realtime_rs::{ChannelEvent, RealtimeChannelOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set in .env file");
    let api_key =
        std::env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set in .env file");

    println!("Creating Supabase client...");
    let client = SupabaseClient::new(&url, &api_key)?;

    #[cfg(feature = "realtime")]
    {
        println!("Connecting to Realtime...");
        let realtime = client.realtime();
        realtime.connect().await?;
        println!("✓ Connected to Realtime!");

        // Create a channel
        println!("\nCreating channel 'room:lobby'...");
        let channel = realtime
            .channel("room:lobby", RealtimeChannelOptions::default())
            .await;

        // Listen for broadcast messages
        let mut rx = channel.on(ChannelEvent::broadcast("message")).await;

        // Subscribe to the channel
        println!("Subscribing to channel...");
        channel.subscribe().await?;
        println!("✓ Subscribed!");

        // Send a message
        println!("\nSending broadcast message...");
        channel
            .send(
                ChannelEvent::broadcast("message"),
                serde_json::json!({
                    "text": "Hello from Rust!",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }),
            )
            .await?;
        println!("✓ Message sent!");

        // Track presence
        println!("\nTracking presence...");
        channel
            .track(serde_json::json!({
                "user": "rust-client",
                "status": "online",
                "joined_at": chrono::Utc::now().to_rfc3339()
            }))
            .await?;
        println!("✓ Presence tracked!");

        // Wait a bit for presence to sync
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Get presence list
        let presence = channel.presence_list().await;
        println!(
            "\nPresent users: {}",
            serde_json::to_string_pretty(&presence)?
        );

        // Listen for messages (with timeout)
        println!("\nListening for messages for 10 seconds...");
        println!("(Try opening another browser tab with the same channel to see messages)");
        let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(10));
        tokio::pin!(timeout);

        let mut message_count = 0;
        loop {
            tokio::select! {
                Some(msg) = rx.recv() => {
                    message_count += 1;
                    println!("📨 Message #{}: {:?}", message_count, msg);
                }
                () = &mut timeout => {
                    println!("\n⏰ Timeout reached after {} messages", message_count);
                    break;
                }
            }
        }

        // Cleanup - handle errors gracefully since connection might have dropped
        println!("\nCleaning up...");

        if let Err(e) = channel.untrack().await {
            println!("⚠️  Could not untrack presence: {}", e);
        } else {
            println!("✓ Presence untracked!");
        }

        if let Err(e) = channel.unsubscribe().await {
            println!("⚠️  Could not unsubscribe: {}", e);
        } else {
            println!("✓ Unsubscribed!");
        }

        if let Err(e) = realtime.disconnect().await {
            println!("⚠️  Could not disconnect cleanly: {}", e);
        } else {
            println!("✓ Disconnected!");
        }

        println!("\n✅ Example completed!");
    }

    #[cfg(not(feature = "realtime"))]
    {
        println!("❌ This example requires the 'realtime' feature to be enabled.");
        println!("Run with: cargo run --example realtime --features realtime");
    }

    Ok(())
}
