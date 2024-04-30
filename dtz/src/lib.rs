pub use dtz_config::*;

#[cfg(feature = "containers")]
/// module for dtz container services
pub mod containers {
    pub use dtz_containers::*;
}

#[cfg(feature = "core")]
/// module for dtz core services
pub mod core {
    pub use dtz_core::*;
}

#[cfg(feature = "identity")]
/// module for dtz identity services
pub mod identity {
    pub use dtz_identity::*;
}

#[cfg(feature = "objectstore")]
/// module for dtz objectstore services
pub mod objectstore {
    pub use dtz_objectstore::*;
}

#[cfg(feature = "observability")]
/// module for dtz observability services
pub mod observability {
    pub use dtz_observability::*;
}

#[cfg(feature = "rss2email")]
/// module for dtz rss2email services
pub mod rss2email {
    pub use dtz_rss2email::*;
}
