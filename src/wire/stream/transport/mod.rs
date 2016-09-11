pub use self::simple::Simple;

pub mod simple;

use Error;
use packet::{PacketKind};
use std::io::prelude::*;

pub trait Transport<K: PacketKind>
{
    fn process_data(&mut self,
                    read: &mut Read) -> Result<(), Error>;

    fn receive_packet(&mut self) -> Result<Option<K>, Error>;

    fn send_packet(&mut self,
                   write: &mut Write,
                   packet: &K) -> Result<(), Error>;
}

