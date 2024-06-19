include!(concat!("../protocol/output", "/protocol.rs"));

pub mod prelude {
    pub use crate::protocol::services::*;
}
