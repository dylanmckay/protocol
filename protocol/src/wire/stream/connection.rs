use {Parcel, Error, Settings};
use hint;
use wire::stream::{Transport, transport};
use wire::middleware;

use std::io::prelude::*;
use std::io::Cursor;

/// A stream-based connection.
// TODO: Allow custom transports.
#[derive(Debug)]
pub struct Connection<P: Parcel, S: Read + Write, M: middleware::Pipeline = middleware::pipeline::Default>
{
    pub stream: S,
    pub transport: transport::Simple,
    pub middleware: M,
    pub settings: Settings,

    pub _a: ::std::marker::PhantomData<P>,
}

impl<P,S,M> Connection<P,S,M>
    where P: Parcel, S: Read + Write, M: middleware::Pipeline
{
    /// Creates a new connection.
    pub fn new(stream: S,
               middleware: M,
               settings: Settings) -> Self {
        Connection {
            stream: stream,
            transport: transport::Simple::new(),
            middleware: middleware,
            settings,
            _a: ::std::marker::PhantomData,
        }
    }

    /// Processes any incoming data in the stream.
    pub fn process_incoming_data(&mut self) -> Result<(), Error> {
        self.transport.process_data(&mut self.stream, &self.settings)
    }

    /// Attempts to receive a packet.
    pub fn receive_packet(&mut self) -> Result<Option<P>, Error> {
        self.process_incoming_data()?;

        if let Some(raw_packet) = self.transport.receive_raw_packet()? {
            let mut packet_data = Cursor::new(self.middleware.decode_data(raw_packet)?);

            let packet = P::read(&mut packet_data, &self.settings, &mut hint::Hints::default())?;

            Ok(Some(packet))
        } else {
            Ok(None)
        }
    }

    /// Sends a packet.
    pub fn send_packet(&mut self, packet: &P) -> Result<(), Error> {
        let raw_packet = self.middleware.encode_data(packet.raw_bytes(&self.settings)?)?;
        self.transport.send_raw_packet(&mut self.stream, &raw_packet, &self.settings)
    }

    pub fn into_inner(self) -> S { self.stream }
}

