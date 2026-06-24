use buffalo_core::{Engine, get_input_method};
use zbus::zvariant::OwnedObjectPath;
use zbus::{Connection, interface};

use crate::config::load_config;
use crate::engine::IBusEngine;

/// The main IBus Factory instance responsible for creating `IBusEngine` sessions.
pub struct IBusFactory {
    /// The session connection.
    pub connection: Connection,
}

impl IBusFactory {
    /// Creates a new `IBusFactory` instance.
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

#[interface(name = "org.freedesktop.IBus.Factory")]
impl IBusFactory {
    /// D-Bus method invoked by IBus to spawn a new input method engine layout.
    async fn create_engine(
        &self,
        engine_name: String,
    ) -> Result<OwnedObjectPath, zbus::fdo::Error> {
        println!("Creating engine for layout: {}", engine_name);

        let config = load_config();
        let active_layout = &config.input_method;
        let im_def = get_input_method(active_layout).ok_or_else(|| {
            zbus::fdo::Error::Failed(format!("Input method {} not found", active_layout))
        })?;
        let engine = Engine::new(im_def, config.flags);
        let ibus_engine = IBusEngine::new(engine);

        let path = format!(
            "/org/freedesktop/IBus/Engine/buffalo/{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let obj_path =
            OwnedObjectPath::try_from(path).map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        self.connection
            .object_server()
            .at(&obj_path, ibus_engine)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(e.to_string()))?;

        Ok(obj_path)
    }
}
