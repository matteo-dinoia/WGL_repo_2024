use wg_network::{NodeId, SourceRoutingHeader};

#[derive(Debug)]
pub struct Message {
    pub message_data: MessageData,
    pub routing_header: SourceRoutingHeader,
}

// Only part fragmentized
#[derive(Debug)]
pub struct MessageData {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: MessageContent,
}

#[derive(Debug)]
pub enum MessageContent {
    // Client -> Server
    ReqServerType,
    ReqFilesList,
    ReqFile(u64),
    ReqMedia(u64),

    ReqClientList,
    ReqRegistrationToChat,
    ReqMessageSend { to: NodeId, message: Vec<u8> },

    // Server -> Client
    RespServerType(ServerType),
    RespFilesList(Vec<u64>),
    RespFile(Vec<u8>),
    RespMedia(Vec<u8>),
    ErrUnsupporedRequestType,
    ErrRequestedNotFound,

    RespClientList(Vec<NodeId>),
    RespMessageFrom { from: NodeId, message: Vec<u8> },
    ErrWrongClientId,
}

#[derive(Debug)]
pub enum ServerType {
    Chat,
    Text,
    Media,
}