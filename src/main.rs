use std::process;

use migrate::console::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("Error occurred: {}", e);
        process::exit(1);
    };
}
