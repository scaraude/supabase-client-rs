//! Example showing various PostgREST query operations.
//!
//! Run with:
//! ```bash
//! SUPABASE_URL=https://your-project.supabase.co \
//! SUPABASE_KEY=your-anon-key \
//! cargo run --example query
//! ```

use supabase_rs::{create_client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let url = std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let key = std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set");

    let client = create_client(&url, &key)?;

    // =========================================================================
    // SELECT queries
    // =========================================================================

    // Basic select
    let _query = client.from("users").select("*");

    // Select specific columns
    let _query = client.from("users").select("id, name, email");

    // Select with relations (foreign key joins)
    let _query = client.from("posts").select("id, title, author:users(name, email)");

    // =========================================================================
    // Filters
    // =========================================================================

    // Equality
    let _query = client.from("users").select("*").eq("status", "active");

    // Not equal
    let _query = client.from("users").select("*").neq("role", "admin");

    // Greater than / Less than
    let _query = client
        .from("products")
        .select("*")
        .gt("price", "100")
        .lt("price", "500");

    // Greater than or equal / Less than or equal
    let _query = client.from("orders").select("*").gte("quantity", "10").lte("quantity", "100");

    // Pattern matching
    let _query = client.from("users").select("*").like("email", "%@gmail.com");

    // Case-insensitive pattern matching
    let _query = client.from("users").select("*").ilike("name", "%john%");

    // Is null / Is not null
    let _query = client.from("users").select("*").is("deleted_at", "null");

    // In array
    let _query = client
        .from("users")
        .select("*")
        .in_("status", vec!["active", "pending"]);

    // =========================================================================
    // Full-text search
    // =========================================================================

    let _query = client
        .from("posts")
        .select("*")
        .fts("content", "rust programming", Some("english"));

    // =========================================================================
    // Ordering and pagination
    // =========================================================================

    let _query = client
        .from("posts")
        .select("*")
        .order("created_at.desc")
        .limit(10)
        .range(0, 9);

    // =========================================================================
    // INSERT
    // =========================================================================

    // Single insert
    let _query = client
        .from("users")
        .insert(r#"{"name": "Alice", "email": "alice@example.com"}"#);

    // Bulk insert
    let _query = client.from("users").insert(
        r#"[
            {"name": "Bob", "email": "bob@example.com"},
            {"name": "Charlie", "email": "charlie@example.com"}
        ]"#,
    );

    // =========================================================================
    // UPDATE
    // =========================================================================

    let _query = client
        .from("users")
        .update(r#"{"status": "inactive"}"#)
        .eq("id", "123");

    // =========================================================================
    // UPSERT
    // =========================================================================

    let _query = client
        .from("users")
        .upsert(r#"{"id": "123", "name": "Alice Updated"}"#);

    // =========================================================================
    // DELETE
    // =========================================================================

    let _query = client.from("users").delete().eq("status", "deleted");

    // =========================================================================
    // RPC (Stored Procedures)
    // =========================================================================

    let _query = client.rpc("get_user_stats", r#"{"user_id": "123"}"#);

    // =========================================================================
    // Execute a real query (uncomment to test)
    // =========================================================================

    // let response = client
    //     .from("your_table")
    //     .select("*")
    //     .limit(5)
    //     .execute()
    //     .await?;
    //
    // match response.status().is_success() {
    //     true => {
    //         let body = response.text().await.unwrap_or_default();
    //         let data: serde_json::Value = serde_json::from_str(&body)?;
    //         println!("{}", serde_json::to_string_pretty(&data)?);
    //     }
    //     false => {
    //         eprintln!("Error: {} - {}", response.status(), response.text().await.unwrap_or_default());
    //     }
    // }

    println!("✓ All query examples compiled successfully!");
    println!("  Uncomment the execute section to run against your Supabase project.");

    Ok(())
}
