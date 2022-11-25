use std::{io::Cursor};
use prost::Message;
use multicast_socket::{MulticastSocket};

use std::thread::{spawn};
use std::sync::{Arc, Mutex};

use std::net::{SocketAddrV4, UdpSocket};

use lazy_static::lazy_static;

// use serialport::{SerialPort, SerialPortType};
// use std::time::Duration;

pub mod fira_protos {
    include!(concat!(env!("OUT_DIR"), "/fira_message.rs"));
}

pub mod ref_protos {
    include!(concat!(env!("OUT_DIR"), "/vssref.rs"));
}

pub mod ssl_vision_protos {
    include!(concat!(env!("OUT_DIR"), "/_.rs"));
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

    pub static ref SOCKET_SSLVISION: MulticastSocket = {
        let socket = SocketAddrV4::new([224, 5, 23, 2].into(), 10006);
        let socket = MulticastSocket::all_interfaces(socket)
            .expect("could not create and bind socket");
            
        socket
    };

    pub static ref FIRASIM: FIRASim = {
        let sim = FIRASim::new();
        sim.start();
        sim
    };

    pub static ref REFEREE: Referee = {
        let referee = Referee::new();
        referee.start();
        referee
    };

    pub static ref SSLVISION: SSLVision = {
        let ssl_vision = SSLVision::new();
        ssl_vision.start();
        ssl_vision
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

fn deserialize_ssl(data: &[u8]) -> Result<ssl_vision_protos::SslWrapperPacket, prost::DecodeError> {
    let mut cursor = Cursor::new(data);
    let ssl_cmd = ssl_vision_protos::SslWrapperPacket::decode(&mut cursor)?;
    Ok(ssl_cmd)
}

fn serialize_packet(packet: fira_protos::Packet) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(packet.encoded_len());

    packet.encode(&mut buf).unwrap();
    buf
}

impl fira_protos::Command {
    pub fn new(id: u32, yellowteam: bool, wheel_left: f64, wheel_right: f64) -> fira_protos::Command {
        fira_protos::Command {
            id,
            yellowteam,
            wheel_left,
            wheel_right,
        }
    }
}

impl fira_protos::Commands {
    pub fn new(commands: Vec<fira_protos::Command>) -> fira_protos::Commands {
        fira_protos::Commands { robot_commands : commands }
    }
}


pub struct FIRASim {
    env: Arc<Mutex<fira_protos::Environment>>,
}

impl FIRASim {
    
    pub fn new() -> FIRASim {
        let empty_env = fira_protos::Environment::default();

        FIRASim {
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

    pub fn send_command(&self, commands: Vec<fira_protos::Command>) {
        {
            let commands = fira_protos::Commands::new(commands);

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
        let empty_frame = fira_protos::Frame::default();

        match self.environment().frame {
            Some(frame) => frame.clone(),
            None => empty_frame,
        }
    }

    pub fn ball(&self) -> fira_protos::Ball {
        let empty_ball = fira_protos::Ball::default();

        match self.frame().ball {
            Some(ball) => ball.clone(),
            None => empty_ball,
        }
    }

    pub fn yellow_robots(&self) -> Vec<fira_protos::Robot> {
        self.frame().robots_blue.clone()
    }

    pub fn blue_robots(&self) -> Vec<fira_protos::Robot> {
        self.frame().robots_yellow.clone()
    }

    pub fn yellow_robot(&self, id: &u32) -> fira_protos::Robot {
        let empty_robot = fira_protos::Robot::default();

        let yellow_robots = self.frame().robots_yellow;

        match  yellow_robots.iter().find(|robot| robot.robot_id == *id) {
            Some(robot) => robot.clone(),
            None => empty_robot,
            
        }
    }

    pub fn blue_robot(&self, id: &u32) -> fira_protos::Robot {
        let empty_robot = fira_protos::Robot::default();

        let blue_robots = self.frame().robots_blue;

        match  blue_robots.iter().find(|robot| robot.robot_id == *id) {
            Some(robot) => robot.clone(),
            None => empty_robot,
        }
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


pub struct SSLVision {
    wrapper: Arc<Mutex<ssl_vision_protos::SslWrapperPacket>>,
}

impl SSLVision {
    
    pub fn new() -> SSLVision {
        let empty_wrapper = ssl_vision_protos::SslWrapperPacket {
            detection: None,
            geometry: None,
        };
        
        SSLVision {
            wrapper: Arc::new(Mutex::new(empty_wrapper))
        }
    }

    pub fn start(&self) {
        let wrapper = self.wrapper.clone();

        spawn(move || {
            loop {
                if let Ok(message) = SOCKET_SSLVISION.receive() {
                    let mut wrapper = wrapper.lock().unwrap();

                    let wrapper_message = match deserialize_ssl(&message.data) {
                       Ok(wrapper) => wrapper,
                       Err(_) => continue
                    };
                    
                    *wrapper = wrapper_message;
                }
            }
        });
    }

    pub fn wrapper(&self) -> ssl_vision_protos::SslWrapperPacket {
        let locked_value = self.wrapper.lock().unwrap();

        locked_value.clone()
    }

    pub fn detection(&self) -> ssl_vision_protos::SslDetectionFrame {
        let empty_detection = ssl_vision_protos::SslDetectionFrame {
            frame_number: 0,
            t_capture: 0.0,
            t_sent: 0.0,
            camera_id: 0,
            balls: Vec::new(),
            robots_yellow: Vec::new(),
            robots_blue: Vec::new(),
        };

        match self.wrapper().detection {
            Some(detection) => detection.clone(),
            None => empty_detection,
        }
    }

    pub fn ball(&self) -> ssl_vision_protos::SslDetectionBall {
        let empty_ball = ssl_vision_protos::SslDetectionBall {
            confidence: 0.0,
            area: None,
            x: 0.0,
            y: 0.0,
            z: None,
            pixel_x: 0.0,
            pixel_y: 0.0,
        };

        match self.detection().balls.first() {
            Some(ball) => ball.clone(),
            None => empty_ball,
        }
    }

    pub fn yellow_robots(&self) -> Vec<ssl_vision_protos::SslDetectionRobot> {
        self.detection().robots_yellow.clone()
    }

    pub fn blue_robots(&self) -> Vec<ssl_vision_protos::SslDetectionRobot> {
        self.detection().robots_blue.clone()
    }

    pub fn yellow_robot(&self, id: &u32) -> ssl_vision_protos::SslDetectionRobot {
        let empty_robot = ssl_vision_protos::SslDetectionRobot {
            confidence: 0.0,
            robot_id: None,
            x: 0.0,
            y: 0.0,
            orientation: None,
            pixel_x: 0.0,
            pixel_y: 0.0,
            height: None,
        };

        let yellow_robots = self.detection().robots_yellow;

        match  yellow_robots.iter().find(|robot| robot.robot_id == Some(*id)) {
            Some(robot) => robot.clone(),
            None => empty_robot,
        }
    }

    pub fn blue_robot(&self, id: &u32) -> ssl_vision_protos::SslDetectionRobot {
        let empty_robot = ssl_vision_protos::SslDetectionRobot {
            confidence: 0.0,
            robot_id: None,
            x: 0.0,
            y: 0.0,
            orientation: None,
            pixel_x: 0.0,
            pixel_y: 0.0,
            height: None,
        };

        let blue_robots = self.detection().robots_blue;

        match  blue_robots.iter().find(|robot| robot.robot_id == Some(*id)) {
            Some(robot) => robot.clone(),
            None => empty_robot,
        }
    }
}

// pub struct Serial {}

// impl Serial {
//     pub fn new() -> Serial {
//         let port = serialport::new("/dev/ttyUSB0", 9600)
//             .timeout(Duration::from_millis(10))
//             .open().expect("Failed to open port");

//         Serial { 
//             port: *port
//          }
//     }

//     pub fn send(&mut self, speed: &[u8; 2]) {
//         let data: [u8; 4] = [5, speed[0]+127, speed[1]+127, 15]; 
//         self.port.write(&data).expect("Failed to write to port");
//     }
// }