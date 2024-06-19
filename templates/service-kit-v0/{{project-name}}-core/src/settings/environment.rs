use clap::ValueEnum;
use config::ValueKind;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, strum::Display, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl Into<ValueKind> for Environment {
    fn into(self) -> ValueKind {
        self.to_string().into()
    }
}
