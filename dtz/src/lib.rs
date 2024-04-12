pub use dtz_config::*;

#[cfg(feature = "containers")]
pub mod containers {
    pub use dtz_containers::*;
}

#[cfg(feature = "core")]
pub mod core {
    pub use dtz_core::*;
}

#[cfg(feature = "identity")]
pub mod identity {
    pub use dtz_identity::*;
}

#[cfg(feature = "objectstore")]
pub mod objectstore {
    pub use dtz_objectstore::*;
}

#[cfg(feature = "observability")]
pub mod observability {
    pub use dtz_observability::*;
}

#[cfg(feature = "rss2email")]
pub mod rss2email {
    pub use dtz_rss2email::*;
}
