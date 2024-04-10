pub use dtz_config::*;

#[cfg(feature = "containers")]
mod containers {
    pub use dtz_containers::*;
}

#[cfg(feature = "core")]
mod core {
    pub use dtz_core::*;
}

#[cfg(feature = "identity")]
mod identity {
    pub use dtz_identity::*;
}

#[cfg(feature = "objectstore")]
mod objectstore {
    pub use dtz_objectstore::*;
}

#[cfg(feature = "observability")]
mod observability {
    pub use dtz_observability::*;
}

#[cfg(feature = "rss2email")]
mod rss2email {
    pub use dtz_rss2email::*;
}
