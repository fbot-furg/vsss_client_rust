
use std::io::Cursor;
use fira_protos::Environment;
use prost::Message;
use std::net::{IpAddr, UdpSocket};
use multicast_socket::MulticastSocket;
use crate::fira_protos::Commands;

use std::net::SocketAddrV4;

pub mod fira_protos;

const VISION_ADDRS: &str = "224.0.0.1:10002";
const COMMAND_ADDRS: &str = "127.0.0.1:20011";


// Create e class Communication as a Singleton

pub struct Communication {
    socket: MulticastSocket,
    environment: Option<Environment>,
}

impl Communication {
    pub fn new() -> Communication {
        let mdns_multicast_address = SocketAddrV4::new([224, 0, 0, 1].into(), 10002);
        let socket = MulticastSocket::all_interfaces(mdns_multicast_address)
            .expect("could not create and bind socket");

        Communication {
            socket,
            environment: None,
        }
    }

    pub fn start(&mut self) {
        loop {
            if let Ok(message) = self.socket.receive() {
                let env = deserialize_env(&message.data).unwrap();
                println!("Received environment {:?}", env);
                self.environment = Some(env);
            };
        }
    }
}

pub fn serialize_packet(packet: &fira_protos::Packet) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(packet.encoded_len());
    
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    packet.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_env(buf: &[u8]) -> Result<fira_protos::Environment, prost::DecodeError> {
    fira_protos::Environment::decode(&mut Cursor::new(buf))
}

pub fn send_command(commands: fira_protos::Commands) {
    {
        let socket_sender = UdpSocket::bind(VISION_ADDRS).unwrap();

        let packet = fira_protos::Packet {
            cmd: Some(commands),
            replace: None        
        };
        let buf = serialize_packet(&packet); 

        match socket_sender.send_to(&buf, COMMAND_ADDRS) {
            Ok(_) => {},
            Err(e) => {
                println!("Error Send {}", e)
            }
        };
    }
}

fn frame() -> Option<fira_protos::Frame>{
    let env = {
        let mut buf = [0; 1024];

        let socket = UdpSocket::bind(VISION_ADDRS).unwrap();

        let (len, _) = socket.recv_from(&mut buf).unwrap();    

        deserialize_env(&buf[..len]).unwrap()
    };

    
    env.frame
}

pub fn ball() -> fira_protos::Ball {
    let mut ret = fira_protos::Ball{
        x: 0.0,
        y: 0.0,
        z: 0.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
    };

    if let Some(frame) = frame() {
        if let Some(ball) = frame.ball {
            ret = ball
        }
    }

    ret
}

pub fn blue_robot(id: &u32) -> Option<fira_protos::Robot> {
    if let Some(frame) = frame() {
        let mut ret = None;

        for robot in frame.robots_blue {
            if robot.robot_id == *id {
                ret = Some(robot)
            }
        };

        ret

    } else { None }
}

pub fn yellow_robot(id: &u32) -> Option<fira_protos::Robot> {
    if let Some(frame) = frame() {
        let mut ret = None;

        for robot in frame.robots_yellow {
            if robot.robot_id == *id {
                ret = Some(robot)
            }
        };

        ret

    } else { 
        None 
    }
}

