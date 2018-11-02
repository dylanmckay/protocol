//! A middleware for compressing all transmitted data.

use Error;
use wire;
use flate2;

use std::io::prelude::*;
use std::io::Cursor;

/// Defines a compression algorithm.
#[derive(Copy, Clone, Debug)]
pub enum Algorithm
{
    /// The zlib compression algorithm.
    ///
    /// https://en.wikipedia.org/wiki/Zlib
    Zlib,
}

/// Compression middleware.
#[derive(Clone, Debug)]
pub enum Compression
{
    /// No compression or decompression should be applied to the data.
    Disabled,
    /// Compression and decompression should be applied to the data.
    Enabled(Algorithm),
}

impl wire::Middleware for Compression
{
    fn encode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Error> {
        match *self {
            Compression::Enabled(algorithm) => match algorithm {
                Algorithm::Zlib => {
                    let e = flate2::write::ZlibEncoder::new(data, flate2::Compression::best());
                    Ok(e.finish()?)
                },
            },
            Compression::Disabled => Ok(data),
        }
    }

    /// Un-processes some data.
    fn decode_data(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Error> {
        match *self {
            Compression::Enabled(algorithm) => match algorithm {
                Algorithm::Zlib => {
                    let d = flate2::read::ZlibDecoder::new(Cursor::new(data));
                    let bytes: Result<Vec<u8>, _> = d.bytes().collect();
                    Ok(bytes?)
                },
            },
            Compression::Disabled => Ok(data),
        }
    }
}

