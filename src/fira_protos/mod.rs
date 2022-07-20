#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ball {
    #[prost(double, tag="1")]
    pub x: f64,
    #[prost(double, tag="2")]
    pub y: f64,
    #[prost(double, tag="3")]
    pub z: f64,
    #[prost(double, tag="4")]
    pub vx: f64,
    #[prost(double, tag="5")]
    pub vy: f64,
    #[prost(double, tag="6")]
    pub vz: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Robot {
    #[prost(uint32, tag="1")]
    pub robot_id: u32,
    #[prost(double, tag="2")]
    pub x: f64,
    #[prost(double, tag="3")]
    pub y: f64,
    #[prost(double, tag="4")]
    pub orientation: f64,
    #[prost(double, tag="5")]
    pub vx: f64,
    #[prost(double, tag="6")]
    pub vy: f64,
    #[prost(double, tag="7")]
    pub vorientation: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(double, tag="1")]
    pub width: f64,
    #[prost(double, tag="2")]
    pub length: f64,
    #[prost(double, tag="3")]
    pub goal_width: f64,
    #[prost(double, tag="4")]
    pub goal_depth: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frame {
    #[prost(message, optional, tag="1")]
    pub ball: ::core::option::Option<Ball>,
    #[prost(message, repeated, tag="2")]
    pub robots_yellow: ::prost::alloc::vec::Vec<Robot>,
    #[prost(message, repeated, tag="3")]
    pub robots_blue: ::prost::alloc::vec::Vec<Robot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(uint32, tag="1")]
    pub id: u32,
    #[prost(bool, tag="2")]
    pub yellowteam: bool,
    #[prost(double, tag="6")]
    pub wheel_left: f64,
    #[prost(double, tag="7")]
    pub wheel_right: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commands {
    #[prost(message, repeated, tag="1")]
    pub robot_commands: ::prost::alloc::vec::Vec<Command>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotReplacement {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Robot>,
    #[prost(bool, tag="5")]
    pub yellowteam: bool,
    #[prost(bool, tag="6")]
    pub turnon: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BallReplacement {
    #[prost(double, tag="1")]
    pub x: f64,
    #[prost(double, tag="2")]
    pub y: f64,
    #[prost(double, tag="3")]
    pub vx: f64,
    #[prost(double, tag="4")]
    pub vy: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replacement {
    #[prost(message, optional, tag="1")]
    pub ball: ::core::option::Option<BallReplacement>,
    #[prost(message, repeated, tag="2")]
    pub robots: ::prost::alloc::vec::Vec<RobotReplacement>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(message, optional, tag="1")]
    pub cmd: ::core::option::Option<Commands>,
    #[prost(message, optional, tag="2")]
    pub replace: ::core::option::Option<Replacement>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    #[prost(uint32, tag="1")]
    pub step: u32,
    #[prost(message, optional, tag="2")]
    pub frame: ::core::option::Option<Frame>,
    #[prost(message, optional, tag="3")]
    pub field: ::core::option::Option<Field>,
    #[prost(uint32, tag="4")]
    pub goals_blue: u32,
    #[prost(uint32, tag="5")]
    pub goals_yellow: u32,
}