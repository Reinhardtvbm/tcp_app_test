use std::net::TcpStream;
use std::io::prelude::*;

#[repr(u8)]
enum State {
    Disconnected = 0b00,
    Calibrate = 0b01,
    Race = 0b10,
}

/// Colours are offset by 0b1000
#[repr(u8)]
enum Colour {
    Black = 0b1000,
    Blue = 0b1001,
    Green = 0b1010,
    Red = 0b1011,
}

#[derive(Debug)]
enum CommsErr {
    WriteFail,
    ReadFail,
    InvalidPacketType,
}

#[repr(u8)]
#[derive(Debug)]
enum PacketType {
    StateChange = 0xF0,
}

#[derive(Debug)]
struct Packet {
    message_type: PacketType,
    message_byte: u8,
}

struct System {
    state: State,
    colour_to_follow: Colour,
    tcp_stream: TcpStream,
}

impl System {
    fn new() -> Option<Self> {
        let tcp_stream = TcpStream::connect(ADDRESS);

        if let Ok(tcp_stream) = tcp_stream {
            Some(Self { state: State::Disconnected, colour_to_follow: Colour::Green, tcp_stream })
        } else {
            None
        }
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), CommsErr> {
        if let Err(_) = self.tcp_stream.write(&[byte]) {
            Err(CommsErr::WriteFail)
        } else {
            Ok(())
        }
    }

    fn read_byte(&mut self) -> Result<u8, CommsErr> {
        let mut input_buffer = [0];
        if let Ok(_) = self.tcp_stream.read(&mut input_buffer){
            Ok(input_buffer[0])
        } else {
            Err(CommsErr::ReadFail) 
        }
    }

    fn read_packet(&mut self) -> Result<Packet, CommsErr> {
        let mut input_buffer = [0; 2];

        if let Ok(_) = self.tcp_stream.read(&mut input_buffer) {
            match (input_buffer[0], input_buffer[1]) {
                (0xF0, b) => Ok(Packet { message_type: PacketType::StateChange, message_byte: b }), 
                (_, _) => Err(CommsErr::InvalidPacketType),
            }
        } else {
            Err(CommsErr::ReadFail) 
        }
    }
}

const ADDRESS: &str = "127.0.0.0:6969";
const CONNECT_REQUEST: u8 = 0xFF;
const CONNECT_ACK: u8 = 0xFE;

fn main() {
    if let Some(mut system) = System::new() {
        system.write_byte(CONNECT_REQUEST).unwrap();
        
        let input_byte = system.read_byte().unwrap();

        if input_byte == CONNECT_ACK {
            if let Ok(packet) = system.read_packet() {
                println!("{:?}", packet);
            }
        } else {
            panic!("Our connection was not acknowledged :(");
        }
    } else {
        panic!("Could not connect tcp stream :/");
    }
}
