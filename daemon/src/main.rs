// IBus Buffalo Input Method Daemon

use buffalo_core::{Config, Engine, InputMethod, Mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ibus-buffalo daemon...");

    let config = Config {
        mode: Mode::Vietnamese,
        method: InputMethod::Telex,
        std_tone: true,
        free_marking: true,
    };
    let mut engine = Engine::new(config);

    let processed = engine.process_key('a');
    println!("Processed text: {}", processed);

    println!("Daemon stopped.");
    Ok(())
}
