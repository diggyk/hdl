#[deny(missing_docs)]

/// Main HDL library

/// Communication Handler
pub mod commhandler;

/// Handle incoming messages
pub mod msgserver;

/// The Registration server module
pub mod regserver;
/// The user interface module
pub mod ui;

/// gRPC protobufs
pub mod protos {
    tonic::include_proto!("messages");
    tonic::include_proto!("register");
}
