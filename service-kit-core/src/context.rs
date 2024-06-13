#[derive(Clone, Debug)]
pub struct WebContext {
    pub settings: crate::settings::Settings,
    pub network: crate::settings::network_settings::NetworkSettings,
    pub storage: crate::storage::StorageCollection,
}

impl WebContext {
    pub async fn new(
        network: crate::settings::network_settings::NetworkSettings,
        settings: crate::settings::Settings,
    ) -> crate::Result<Self> {
        Ok(Self {
            network,
            storage: crate::storage::StorageCollection::file_index(settings.storage_path()).await?,
            settings,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn listener(&self) -> crate::Result<tokio::net::TcpListener> {
        self.network.listener().await
    }

    pub fn settings(&self) -> &crate::settings::Settings {
        &self.settings
    }
}
