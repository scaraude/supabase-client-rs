//! Basic example showing how to use the Supabase client.
//!
//! Run with:
//! ```bash
//! SUPABASE_URL=https://your-project.supabase.co \
//! SUPABASE_KEY=your-anon-key \
//! cargo run --example basic
//! ```

use supabase_rs::{create_client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Load from environment
    let url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let key = std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

    // Create client
    let client = create_client(&url, &key)?;

    println!("✓ Client created successfully");
    println!("  URL: {}", client.config().url);
    println!("  REST: {}", client.config().rest_url());
    println!("  Realtime: {}", client.realtime_url());

    // Example: Select from a table
    // Uncomment and adjust for your schema:
    //
    // let response = client
    //     .from("users")
    //     .select("*")
    //     .limit(5)
    //     .execute()
    //     .await?;
    //
    // println!("Response status: {}", response.status());
    // println!("Body: {}", response.text().await.unwrap_or_default());

    Ok(())
}
