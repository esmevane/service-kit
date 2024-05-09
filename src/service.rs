use service_manager::*;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct Service {
    label: ServiceLabel,
    manager: Box<dyn ServiceManager>,
}

impl Service {
    pub fn init(label: ServiceLabel) -> crate::Result<Self> {
        let manager = <dyn ServiceManager>::native()?;

        Ok(Self { label, manager })
    }

    pub fn install(&self, program: PathBuf, args: Vec<OsString>) -> crate::Result<()> {
        self.manager.install(ServiceInstallCtx {
            label: self.label.clone(),
            program,
            args,
            contents: None,
            username: None,
            working_directory: None,
            environment: None,
        })?;

        Ok(())
    }

    pub fn start(&self) -> crate::Result<()> {
        self.manager.start(ServiceStartCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }

    pub fn stop(&self) -> crate::Result<()> {
        self.manager.stop(ServiceStopCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }

    pub fn uninstall(&self) -> crate::Result<()> {
        self.manager.uninstall(ServiceUninstallCtx {
            label: self.label.clone(),
        })?;

        Ok(())
    }
}
