#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    NoResponse,
    InvalidCommand,
    InvalidLength,
}

// // 7609 bytes is max message size for ctaphid
// type U6144 = <heapless::consts::U4096 as core::ops::Add<heapless::consts::U2048>>::Output;
// type U7168 = <U6144 as core::ops::Add<heapless::consts::U1024>>::Output;
// pub type U7609 = <U7168 as core::ops::Add<heapless::consts::U441>>::Output;
// pub type U7609 = heapless::consts::U4096;

// TODO: find reasonable size
// pub type Message = heapless::Vec<u8, 3072>;
pub type Message = heapless::Vec<u8, 7609>;
pub type AppResult = core::result::Result<(), Error>;
pub type ShortMessage = heapless::Vec<u8, 1024>;
pub type Request = (Command, Message);
pub type Response = core::result::Result<Message, Error>;

pub use crate::command::Command;
