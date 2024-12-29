use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

/**
 * Represents a message that can be sent between nodes.
 * Contains the source node id, the session id and the content of the message.
 */
#[derive(Debug, Clone)]
pub struct Message<M: DroneSend> {
    pub source_id: NodeId,
    pub session_id: u64,
    pub content: M,
}

/**
 * Serialization/Deserialization of the message
 */
pub trait DroneSend: Serialize + DeserializeOwned {
    fn stringify(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    fn from_string(raw: String) -> Result<Self, String> {
        serde_json::from_str(raw.as_str()).map_err(|e| e.to_string())
    }
}

pub trait Request: DroneSend {}
pub trait Response: DroneSend {}

/**
 * Server type request (Media, Chat, Text)
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerTypeRequest {
    ServerType,
}

impl DroneSend for ServerTypeRequest {}
impl Request for ServerTypeRequest {}

/**
 * Server type response (Media, Chat, Text)
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerTypeResponse {
    ServerType(ServerType),
}

impl DroneSend for ServerTypeResponse {}
impl Response for ServerTypeResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerType {
    Chat,
    Text,
    Media,
}
