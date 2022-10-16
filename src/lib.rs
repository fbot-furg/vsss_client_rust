use std::iter::empty;
use std::{io::Cursor};
use prost::Message;
use multicast_socket::{MulticastSocket};

use std::thread::{spawn};
use std::sync::{Arc, Mutex};

use std::net::{SocketAddrV4, UdpSocket};

use lazy_static::lazy_static;

pub mod fira_protos {
    include!(concat!(env!("OUT_DIR"), "/fira_message.rs"));
}

pub mod ref_protos {
    include!(concat!(env!("OUT_DIR"), "/vssref.rs"));
    
}

lazy_static! {
    pub static ref SOCKET_VISION: MulticastSocket = {
        let socket = SocketAddrV4::new([224, 0, 0, 1].into(), 10002);
        let socket = MulticastSocket::all_interfaces(socket)
            .expect("could not create and bind socket");
            
        socket
    };

    pub static ref SOCKET_REFEREE: MulticastSocket = {
        let socket = SocketAddrV4::new([224, 5, 23, 2].into(), 10003);
        let socket = MulticastSocket::all_interfaces(socket)
            .expect("could not create and bind socket");
            
        socket
    };
}


const VISION_ADDRS: &str = "224.0.0.1:10010";
const COMMAND_ADDRS: &str = "127.0.0.1:20011";

fn deserialize_env(data: &[u8]) -> Result<fira_protos::Environment, prost::DecodeError> {
    let mut cursor = Cursor::new(data);
    let env = fira_protos::Environment::decode(&mut cursor)?;
    Ok(env)
}

fn deserialize_ref(data: &[u8]) -> Result<ref_protos::VssRefCommand, prost::DecodeError> {
    let mut cursor = Cursor::new(data);
    let ref_cmd = ref_protos::VssRefCommand::decode(&mut cursor)?;
    Ok(ref_cmd)
}

fn serialize_packet(packet: fira_protos::Packet) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(packet.encoded_len());

    packet.encode(&mut buf).unwrap();
    buf
}

pub struct FIRASim {
    // socket: MulticastSocket,
    env: Arc<Mutex<fira_protos::Environment>>,
}

impl FIRASim {
    
    pub fn new() -> FIRASim {
        // let socket = SocketAddrV4::new([224, 0, 0, 1].into(), 10002);
        // let socket = MulticastSocket::all_interfaces(socket)
        //     .expect("could not create and bind socket");

        let empty_env = fira_protos::Environment {
            step: 0,
            frame: None,
            field: None,
            goals_blue: 0,
            goals_yellow: 0,
        };

        FIRASim {
            // socket,
            env: Arc::new(Mutex::new(empty_env)),
        }
    }

    pub fn start(&self) {
        let env = self.env.clone();

        spawn(move || {
            loop {
                if let Ok(message) = SOCKET_VISION.receive() {
                    let mut env = env.lock().unwrap();

                    let env_message = deserialize_env(&message.data).unwrap();
                    
                    *env = env_message;
                }
            }
        });
    }

    pub fn send_command(&self, commands: fira_protos::Commands) {
        {
            if let Ok(socket) = UdpSocket::bind(VISION_ADDRS) {
                let packet = fira_protos::Packet {
                    cmd: Some(commands),
                    replace: None        
                };
                let buf = serialize_packet(packet); 
        
                match socket.send_to(&buf, COMMAND_ADDRS) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Error Send {}", e)
                    }
                };
            } else {
                println!("Error Bind");
            }
        }
    }

    pub fn environment(&self) -> fira_protos::Environment {
        let locked_value = self.env.lock().unwrap();
        locked_value.clone()
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
    referee: Arc<Mutex<ref_protos::VssRefCommand>>,
}

impl Referee {
    
    pub fn new() -> Referee {
        let empty_referee = ref_protos::VssRefCommand {
            foul: 0,
            teamcolor: 0,
            foul_quadrant: 0,
            timestamp: 0.0,
            game_half: 0,
        };
        
        Referee {
            referee: Arc::new(Mutex::new(empty_referee))
        }
    }

    pub fn start(&self) {
        let referee = self.referee.clone();

        spawn(move || {
            loop {
                if let Ok(message) = SOCKET_REFEREE.receive() {
                    let mut referee = referee.lock().unwrap();

                    let ref_message = deserialize_ref(&message.data).unwrap();
                    
                    *referee = ref_message;
                }
            }
        });
    }

    pub fn referee(&self) -> ref_protos::VssRefCommand {
        let locked_value = self.referee.lock().unwrap();
        locked_value.clone()
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

