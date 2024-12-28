use crate::messages::general_messages::{DroneSend, Request, Response, ServerTypeResponse};
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

use super::general_messages::ServerTypeRequest;

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
    ClientList(Vec<NodeId>), // Response to a client when the list of clients is requested
    MessageFrom { from: NodeId, message: Vec<u8> }, // Response to a client when a message is received
    MessageSent, // Response to a client when the message is sent successfully
    ClientRegistered, // Response to a client when the registration is successful
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatRequestWrapper {
    Chat(ChatRequest),
    ServerType(ServerTypeRequest),
}

impl Request for ChatRequestWrapper {}
impl DroneSend for ChatRequestWrapper {}
