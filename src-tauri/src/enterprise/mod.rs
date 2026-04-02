#[cfg(feature = "enterprise")]
pub mod license;

#[cfg(feature = "enterprise")]
pub use license::is_enterprise;

#[cfg(not(feature = "enterprise"))]
pub fn is_enterprise() -> bool {
    false
}
