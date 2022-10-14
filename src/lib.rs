use std::{io::Cursor};
use prost::Message;
use multicast_socket::{MulticastSocket, Interface};

use std::net::{SocketAddrV4, UdpSocket};

pub mod fira_protos {
    include!(concat!(env!("OUT_DIR"), "/fira_message.rs"));
}

pub mod ref_protos {
    include!(concat!(env!("OUT_DIR"), "/vssref.rs"));
    
}

const VISION_ADDRS: &str = "224.0.0.1:10010";
const COMMAND_ADDRS: &str = "127.0.0.1:20011";

pub struct FIRASim {
    socket: MulticastSocket,
}

impl FIRASim {
    
    pub fn new() -> FIRASim {
        let socket = SocketAddrV4::new([224, 0, 0, 1].into(), 10002);
        let socket = MulticastSocket::all_interfaces(socket)
            .expect("could not create and bind socket");

        FIRASim {
            socket,
        }
    }

    fn deserialize_env(&self, data: &[u8]) -> Result<fira_protos::Environment, prost::DecodeError> {
        let mut cursor = Cursor::new(data);
        let env = fira_protos::Environment::decode(&mut cursor)?;
        Ok(env)
    }

    fn serialize_packet(&self, packet: fira_protos::Packet) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());

        packet.encode(&mut buf).unwrap();
        buf
    }

    pub fn send_command(&self, commands: fira_protos::Commands) {
        {
            let socket_sender = UdpSocket::bind(VISION_ADDRS).unwrap();
    
            let packet = fira_protos::Packet {
                cmd: Some(commands),
                replace: None        
            };
            let buf = self.serialize_packet(packet); 
    
            match socket_sender.send_to(&buf, COMMAND_ADDRS) {
                Ok(_) => {},
                Err(e) => {
                    println!("Error Send {}", e)
                }
            };
        }
    }

    pub fn environment(&self) -> fira_protos::Environment {
        if let Ok(message) = self.socket.receive() {
            let env = self.deserialize_env(&message.data).unwrap();
            return env;
        };

        fira_protos::Environment {
            step: 0,
            frame: None,
            field: None,
            goals_blue: 0,
            goals_yellow: 0,
        }
    }


    pub fn frame(&self) -> fira_protos::Frame{
        let empty_frame = fira_protos::Frame {
            ball: None,
            robots_yellow: Vec::new(),
            robots_blue: Vec::new(),
        };

        match self.environment().frame {
            Some(frame) => frame.clone(),
            None => empty_frame,
        }
    }

    pub fn ball(&self) -> fira_protos::Ball {
        let empty_ball = fira_protos::Ball {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        };

        match self.frame().ball {
            Some(ball) => ball.clone(),
            None => empty_ball,
        }
    }

    pub fn yellow_robot(&self, id: &u32) -> Option<fira_protos::Robot> {
        let mut robot = None;
        for r in self.frame().robots_yellow {
            if r.robot_id == *id {
                robot = Some(r);
            }
        }
        robot
    }

    pub fn blue_robot(&self, id: &u32) -> Option<fira_protos::Robot> {
        let mut robot = None;
        for r in self.frame().robots_blue {
            if r.robot_id == *id {
                robot = Some(r);
            }
        }
        robot
    }

}

pub struct Referee {
    socket: MulticastSocket,
}

impl Referee {
    
    pub fn new() -> Referee {
        let mdns_multicast_address = SocketAddrV4::new([224, 5, 23, 2].into(), 10003);
        let socket = MulticastSocket::all_interfaces(mdns_multicast_address)
            .expect("could not create and bind socket");

        Referee {
            socket,
        }
    }

    fn deserialize_ref(&self, data: &[u8]) -> Result<ref_protos::VssRefCommand, prost::DecodeError> {
        let mut cursor = Cursor::new(data);
        let ref_cmd = ref_protos::VssRefCommand::decode(&mut cursor)?;
        Ok(ref_cmd)
    }

    pub fn referee(&self) -> ref_protos::VssRefCommand {
        if let Ok(message) = self.socket.receive() {
            let referee = self.deserialize_ref(&message.data).unwrap();
            return referee;
        };

        ref_protos::VssRefCommand {
            foul: 0,
            teamcolor: 0,
            foul_quadrant: 0,
            timestamp: 0.0,
            game_half: 0,
        }
    }

    pub fn foul(&self) -> ref_protos::Foul {
        match self.referee().foul {
            0 => ref_protos::Foul::FreeKick,
            1 => ref_protos::Foul::PenaltyKick,
            2 => ref_protos::Foul::GoalKick,
            3 => ref_protos::Foul::FreeBall,
            4 => ref_protos::Foul::Kickoff,
            5 => ref_protos::Foul::Stop,
            6 => ref_protos::Foul::GameOn,
            7 => ref_protos::Foul::Halt,
            _ => ref_protos::Foul::FreeKick,
        }
    }
}

