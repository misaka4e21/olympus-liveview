use std::fmt;
use std::io::Write;
use std::net::UdpSocket;

enum PacketType {
    First,
    Middle,
    End,
}
impl fmt::Display for PacketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PacketType::First => "FirstPacket",
            PacketType::Middle => "MiddlePacket",
            PacketType::End => "EndPacket",
        })
    }
}

struct Frame {
    pub packet_type: PacketType,
    pub stream_number: u32,
    pub frame_number: u32,
    pub chunk_number: u16,
    pub buffer: Vec<u8>,
}

fn u8_to_u16(buf: &[u8]) -> u16 {
    (buf[0] as u16) * 0x100 + (buf[1] as u16)
}

fn u8_to_u32(buf: &[u8]) -> u32 {
    u8_to_u16(&buf[0..2]) as u32 * 0x10000 + u8_to_u16(&buf[2..4]) as u32
}

fn generate_frame(socket: &UdpSocket) -> Option<Frame> {
    let mut buf = [0; 1500];
    match socket.recv_from(&mut buf) {
        Ok((amt, _src)) => {
            // Redeclare `buf` as slice of the received data
            let buf = &mut buf[..amt];

            // Calculate packet type
            let packet_type = match u8_to_u16(&buf[0..2]) {
                0x9060 => PacketType::First,
                0x8060 => PacketType::Middle,
                0x80e0 => PacketType::End,
                _ => return None
            };
            Some(Frame {
                packet_type: packet_type,
                chunk_number: u8_to_u16(&buf[2..4]),
                frame_number: u8_to_u32(&buf[4..8]),
                stream_number: u8_to_u32(&buf[8..12]),
                buffer: Vec::from(buf),
            })
        },
        _ => None
    }
}

fn find_jpeg_start(buf: Vec<u8>) -> Vec<u8> {
    for i in 12..buf.len()-2 {
        if u8_to_u16(&buf[i..i+2]) == 0xffd8 {
            return buf[i..buf.len()].to_vec();
        }
    }
    buf[12..buf.len()].to_vec()
}

struct Picture {
    jpeg: Vec<u8>,
    current_chunk_number: u16,
    stream_number: u32,
    frame_number: u32,
}

impl Picture {
    fn new() -> Picture {
      Picture {
        jpeg: Vec::new(),
        current_chunk_number: 0,
        stream_number: 0,
        frame_number: 0,
      }
    }
    fn add_initial_data(&mut self, frame: Frame) {
        self.current_chunk_number = frame.chunk_number;
        self.frame_number = frame.frame_number;
        self.stream_number = frame.stream_number;
        self.jpeg = find_jpeg_start(frame.buffer);
    }
    fn add_appended_data(&mut self, frame: Frame) {
        if frame.frame_number == self.frame_number {
            self.jpeg = [self.jpeg.clone(), frame.buffer[12..frame.buffer.len()].to_vec()].concat();
        }
    }

    fn add_data(&mut self, frame: Frame) -> bool{
        match frame.packet_type {
            PacketType::First => {
                self.add_initial_data(frame);
                false
            }
            PacketType::End=> {
                self.add_appended_data(frame);
                true 
            }
            _ => {
                self.add_appended_data(frame);
                false
            }
        }
    }

    fn get_data(&self) -> &[u8]{
        self.jpeg.as_slice()
    }
}

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:23333")?;
        let mut picture = Picture::new();
        loop {
            match generate_frame(&socket) {
                Some(frame) => {
                    if picture.add_data(frame) {
                        std::io::stdout().write_all(picture.get_data())?;
                    }
                }
                None => {
                    break
                }
            }
        }
    }
    Ok(())
}
