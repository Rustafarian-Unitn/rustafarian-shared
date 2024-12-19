use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;
use crate::messages::general_messages::{DroneSend, Request, Response, ServerTypeResponse};

/**
 * Request type for a chat client
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatRequest {
    ClientList,
    Register(NodeId),
    SendMessage {
        from: NodeId,
        to: NodeId,
        message: String,
    },
}

impl DroneSend for ChatRequest {}
impl Request for ChatRequest {}

/**
 * Response type for a chat client
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatResponse {
    ClientList(Vec<NodeId>),
    MessageFrom { from: NodeId, message: Vec<u8> },
    MessageSent,
}

impl DroneSend for ChatResponse {}
impl Response for ChatResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatResponseWrapper {
    Chat(ChatResponse),
    ServerType(ServerTypeResponse),
}

impl Response for ChatResponseWrapper {}
impl DroneSend for ChatResponseWrapper {}