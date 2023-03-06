mod bot;
mod chatgpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // Use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;

    let mut client = bot::init_client().await; // Get Initialized Serenity Client

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|e| eprintln!("[ ERROR ] {}", e));
    });

    if let Err(e) = tokio::signal::ctrl_c().await {
        eprintln!("[ ERROR ] {}", e);
    }
    println!("[ SIGNAL ] Received Ctrl+C, Shutting Down.");
    shard_manager.lock().await.shutdown_all().await;

    Ok(())
}
