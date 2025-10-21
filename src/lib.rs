// === ZT30 ===
#[cfg(all(feature = "zt30", feature = "tcp"))]
pub mod zt30_tcp;
#[cfg(all(feature = "zt30", feature = "ttl"))]
pub mod zt30_ttl;
#[cfg(all(feature = "zt30", feature = "udp"))]
pub mod zt30_udp;

// === ZT6 ===
#[cfg(all(feature = "zt6", feature = "tcp"))]
pub mod zt6_tcp;
#[cfg(all(feature = "zt6", feature = "ttl"))]
pub mod zt6_ttl;
#[cfg(all(feature = "zt6", feature = "udp"))]
pub mod zt6_udp;

// === ZR30 ===
#[cfg(all(feature = "zr30", feature = "tcp"))]
pub mod zr30_tcp;
#[cfg(all(feature = "zr30", feature = "ttl"))]
pub mod zr30_ttl;
#[cfg(all(feature = "zr30", feature = "udp"))]
pub mod zr30_udp;

// === ZR10 ===
#[cfg(all(feature = "zr10", feature = "tcp"))]
pub mod zr10_tcp;
#[cfg(all(feature = "zr10", feature = "ttl"))]
pub mod zr10_ttl;
#[cfg(all(feature = "zr10", feature = "udp"))]
pub mod zr10_udp;

// === A8Mini ===
#[cfg(all(feature = "a8mini", feature = "tcp"))]
pub mod a8mini_tcp;
#[cfg(all(feature = "a8mini", feature = "ttl"))]
pub mod a8mini_ttl;
#[cfg(all(feature = "a8mini", feature = "udp"))]
pub mod a8mini_udp;

// === A2Mini ===
#[cfg(all(feature = "a2mini", feature = "tcp"))]
pub mod a2mini_tcp;
#[cfg(all(feature = "a2mini", feature = "ttl"))]
pub mod a2mini_ttl;
#[cfg(all(feature = "a2mini", feature = "udp"))]
pub mod a2mini_udp;
