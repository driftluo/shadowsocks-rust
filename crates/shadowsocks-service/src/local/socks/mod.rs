//! Shadowsocks SOCKS (4/4a, 5) Local Server

pub use self::server::Socks;

pub mod client;
pub mod server;
#[cfg(feature = "local-socks4")]
pub mod socks4;

use once_cell::sync::Lazy;

// Currently, read SOCKS5 username and password from env
static SS_SOCKS_USERNAME: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var_os("SS_SOCKS_USERNAME")
        .map(|val| unsafe {
            let mut val = std::mem::transmute::<std::ffi::OsString, Vec<u8>>(val);
            if val.len() > 255 {
                val.truncate(255);
                val.shrink_to_fit();
            }
            val
        })
        .unwrap_or(vec![])
});
static SS_SOCKS_PASSWORD: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var_os("SS_SOCKS_PASSWORD")
        .map(|val| unsafe {
            let mut val = std::mem::transmute::<std::ffi::OsString, Vec<u8>>(val);
            if val.len() > 255 {
                val.truncate(255);
                val.shrink_to_fit();
            }
            val
        })
        .unwrap_or(vec![])
});
