use serde::{Deserialize, Serialize};
use crate::messages::general_messages::{DroneSend, Request, Response, ServerTypeResponse};

use super::general_messages::ServerTypeRequest;

/**
 * Request type for a chat client
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserRequest {
    FileList,
    TextFileRequest(u8),
    MediaFileRequest(u8),
}

impl DroneSend for BrowserRequest {}
impl Request for BrowserRequest {}

/**
 * Response type for a chat client
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserResponse {
    FileList(Vec<u8>),
    TextFile(u8, String),
    MediaFile(u8, Vec<u8>),
}

impl DroneSend for BrowserResponse {}
impl Response for BrowserResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserResponseWrapper {
    Chat(BrowserResponse),
    ServerType(ServerTypeResponse),
}

impl Response for BrowserResponseWrapper {}
impl DroneSend for BrowserResponseWrapper {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserRequestWrapper {
    Chat(BrowserRequest),
    ServerType(ServerTypeRequest),
}

impl Request for BrowserRequestWrapper {}
impl DroneSend for BrowserRequestWrapper {}