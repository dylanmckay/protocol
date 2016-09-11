pub use self::simple::Simple;

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

pub mod simple
{
    /// The type that we use to describe packet sizes.
    pub type PacketSize = u32;

    use super::Transport;

    use {Error, Type};
    use packet::{PacketKind};

    use std::collections::VecDeque;
    use std::io::prelude::*;
    use std::io::Cursor;
    use std::mem;

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
    pub struct Simple<K: PacketKind>
    {
        state: State,
        packets: VecDeque<K>,
    }

    impl<K: PacketKind> Simple<K>
    {
        pub fn new() -> Self {
            Simple {
                state: State::AwaitingSize(Vec::new()),
                packets: VecDeque::new(),
            }
        }
    }

    impl<K: PacketKind> Transport<K> for Simple<K>
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
                            let mut packet_data = Cursor::new(received_data);
                            let packet = K::read(&mut packet_data)?;
                            self.packets.push_back(packet);

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

        fn send_packet(&mut self,
                       write: &mut Write,
                       packet: &K) -> Result<(), Error> {
            let packet_data = {
                let mut buffer = Cursor::new(Vec::new());
                packet.write(&mut buffer)?;
                buffer.into_inner()
            };

            // Prefix the packet size.
            (packet_data.len() as PacketSize).write(write)?;
            // Write the packet data.
            write.write(&packet_data)?;

            Ok(())
        }

        fn receive_packet(&mut self) -> Result<Option<K>, Error> {
            Ok(self.packets.pop_front())
        }
    }
}

