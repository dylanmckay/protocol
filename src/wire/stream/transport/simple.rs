use super::Transport;

use {Error, Type};

use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::Cursor;
use std::mem;

/// The type that we use to describe packet sizes.
pub type PacketSize = u32;

/// The current state.
#[derive(Clone)]
enum State
{
    /// We are awaiting packet size bytes.
    AwaitingSize(Vec<u8>),
    AwaitingPacket {
        size: PacketSize,
        received_data: Vec<u8>,
    },
}

/// A simple transport.
pub struct Simple
{
    state: State,
    packets: VecDeque<Vec<u8>>,
}

impl Simple
{
    pub fn new() -> Self {
        Simple {
            state: State::AwaitingSize(Vec::new()),
            packets: VecDeque::new(),
        }
    }
}

impl Transport for Simple
{
    fn process_data(&mut self,
                    read: &mut Read) -> Result<(), Error> {
        loop {
            match self.state.clone() {
                State::AwaitingSize(mut size_bytes) => {
                    let remaining_bytes = size_bytes.len() - mem::size_of::<PacketSize>();
                    assert!(remaining_bytes > 0);

                    let mut received_bytes = vec![0; remaining_bytes];
                    let bytes_read = read.read(&mut received_bytes)?;
                    received_bytes.drain(bytes_read..);

                    assert_eq!(received_bytes.len(), bytes_read);

                    size_bytes.extend(received_bytes.into_iter());

                    if size_bytes.len() == mem::size_of::<PacketSize>() {
                        let mut size_buffer = Cursor::new(size_bytes);

                        let size = PacketSize::read(&mut size_buffer).unwrap();

                        // We are now ready to receive packet data.
                        self.state = State::AwaitingPacket { size: size, received_data: Vec::new() }
                    } else {
                        // Still waiting to receive the whole packet.
                        self.state = State::AwaitingSize(size_bytes);
                        break;
                    }
                },
                State::AwaitingPacket { size, mut received_data } => {
                    let remaining_bytes = (size as usize) - received_data.len();
                    assert!(remaining_bytes > 0);

                    let mut received_bytes = vec![0; remaining_bytes];
                    let bytes_read = read.read(&mut received_bytes)?;
                    received_bytes.drain(bytes_read..);

                    assert_eq!(received_bytes.len(), bytes_read);

                    received_data.extend(received_bytes.into_iter());

                    assert!(received_data.len() <= (size as usize));

                    if (size as usize) == received_data.len() {
                        self.packets.push_back(received_data);

                        // Start reading the next packet.
                        self.state = State::AwaitingSize(Vec::new());
                    } else {
                        // Keep reading the current packet.
                        self.state = State::AwaitingPacket { size: size, received_data: received_data };
                        break;
                    }
                },
            }
        }

        Ok(())
    }

    fn send_raw_packet(&mut self,
                       write: &mut Write,
                       packet: &Vec<u8>) -> Result<(), Error> {
        // Prefix the packet size.
        (packet.len() as PacketSize).write(write)?;
        // Write the packet data.
        write.write(&packet)?;

        Ok(())
    }

    fn receive_raw_packet(&mut self) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.packets.pop_front())
    }
}

