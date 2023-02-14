mod bot;

#[tokio::main]
async fn main() {
    let mut client = bot::init_client().await; // Get Initialized Serenity Client

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
}
