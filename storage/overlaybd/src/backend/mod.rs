pub mod cache;
pub mod local;
#[cfg(feature = "full")]
pub mod oss;
#[cfg(feature = "full")]
pub mod registryfs_v2;
pub mod switch;
pub mod tar;

// Backward-compatible path while implementation is aligned to overlaybd registryfs_v2.
#[cfg(feature = "full")]
pub use registryfs_v2 as remote;

/// An HTTP failure from the registry blob reader. Outer cache and image fallbacks
/// propagate it so they cannot restart the reader's retry budget.
#[derive(Debug, Clone, thiserror::Error)]
#[error("{context}: http status {}", .status.as_u16())]
pub(crate) struct RegistryReadError {
    context: String,
    pub(crate) status: http::StatusCode,
    #[cfg(feature = "full")]
    retry_after: Option<std::time::Duration>,
}
