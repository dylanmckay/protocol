use {PacketKind, Error};
use packet::{Transport, transport};

use std::io::prelude::*;

/// A stream-based connection.
// TODO: Allow custom transports.
pub struct Connection<P: PacketKind, S: Read + Write>
{
    pub stream: S,
    pub transport: transport::Simple<P>,
}

impl<P,S> Connection<P,S>
    where P: PacketKind, S: Read + Write
{
    /// Creates a new connection.
    pub fn new(stream: S) -> Self {
        Connection {
            stream: stream,
            transport: transport::Simple::new(),
        }
    }

    /// Processes any incoming data in thes stream.
    pub fn process_incoming_data(&mut self) -> Result<(), Error> {
        self.transport.process_data(&mut self.stream)
    }

    /// Attempts to receive a packet.
    pub fn receive_packet(&mut self) -> Result<Option<P>, Error> {
        self.transport.receive_packet()
    }

    /// Sends a packet.
    pub fn send_packet(&mut self, packet: &P) -> Result<(), Error> {
        self.transport.send_packet(&mut self.stream, packet)
    }

    pub fn into_inner(self) -> S { self.stream }
}

