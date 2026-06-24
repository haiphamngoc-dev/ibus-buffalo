use zbus::connection::Builder;

use crate::factory::IBusFactory;
use crate::utils::get_ibus_address;
use crate::x11_helper;

/// Runs the main IBus Buffalo input method daemon loop.
/// Resolves the IBus address, connects to the D-Bus session, and registers
/// the IBus Factory service.
pub async fn run_daemon() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting ibus-buffalo daemon...");
    x11_helper::set_x_ignore_error_handler();

    let addr = get_ibus_address().expect("Could not find IBus daemon address");
    println!("Connecting to IBus daemon at address: {}", addr);

    let connection = Builder::address(addr.as_str())?.build().await?;

    println!("Requesting name: org.freedesktop.IBus.buffalo");
    connection
        .request_name("org.freedesktop.IBus.buffalo")
        .await?;

    println!("Publishing IBus Factory service at /org/freedesktop/IBus/Factory");
    let factory = IBusFactory {
        connection: connection.clone(),
    };
    connection
        .object_server()
        .at("/org/freedesktop/IBus/Factory", factory)
        .await?;

    println!("Daemon successfully started. Waiting for connections...");
    std::future::pending::<()>().await;

    Ok(())
}
