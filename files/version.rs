//! Static build info for Rpg Maker Mz.

pub const BUILD_VERSION: &str  = "v3.8.7";
pub const BUILD_CHANNEL: &str  = "lts";
pub const BUILD_COMMIT: &str   = "c64e076fdce6";
pub const BUILD_TIMESTAMP: u64 = 1760905542;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Rpg Maker Mz", BUILD_VERSION, BUILD_CHANNEL)
}
