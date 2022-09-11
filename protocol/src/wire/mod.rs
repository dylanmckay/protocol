//! Stream-based and datagram-based communication implementations.
//!
//! Also includes other utilities useful for reading and writing
//! parcels.

pub use self::middleware::Middleware;
pub use self::reader::Reader;

/// Datagram-based over the wire communication.
pub mod dgram;
mod reader;
#[macro_use] pub mod middleware;
/// Stream-based over the wire communication.
pub mod stream;


