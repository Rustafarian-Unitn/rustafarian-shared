use crossbeam_channel::Sender;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wg_2024::{network::NodeId, packet::Packet};

use crate::{
    messages::general_messages::{DroneSend, Request},
    topology::Topology,
};

use super::general_messages::ServerType;

/**
 * Command that can be sent from the simulation controller to the (chat) clients
 */
#[derive(Debug, Clone)]
pub enum SimControllerCommand {
    SendMessage(String, NodeId, NodeId), // Send message to a server, the first id is the server, the second the destination client
    Register(NodeId),                    // Register a client to a server
    ClientList(NodeId),                  // Get the list of available clients from a server
    FloodRequest,                        // Send a flood request
    Topology,                            // Get the topology of the network
    RequestTextFile(u8, NodeId), // Request a text file from the server (filename, server_id)
    RequestMediaFile(u8, NodeId), // Request a media file from the server (filename, server_id)
    RequestFileList(NodeId),     // Request the list of available files from the server
    KnownServers,                // Request the client its list of known servers
    RegisteredServers,           // Request the list of servers to which the client is registered
    RemoveSender(NodeId),        // Remove a sender from the list of neighbors
    AddSender(NodeId, Sender<Packet>), // Add a sender to the list of neighbors
    RequestServerType(NodeId),        // Request the type of a server
}

/**
 * Messages that can be sent from the clients to the simulation controller
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerMessage {
    FloodResponse(u64),                        // Response to a flood request
    TopologyResponse(Topology),                // Response to a topology request
    ClientListResponse(NodeId, Vec<NodeId>), // The client list associated to a server, as the client knows it
    MessageReceived(NodeId, NodeId, String), // A message received by a client (server_id, node_from, message)
    TextFileResponse(u8, String),            // Response to a text file request
    MediaFileResponse(u8, Vec<u8>),          // Response to a media file request
    FileListResponse(Vec<u8>),               // Response to a file list request
    ServerTypeResponse(NodeId, ServerType),  // Response to ServerType request from a client
    KnownServers(HashMap<NodeId, ServerType>), // Response to KnownServers request from a client
    RegisteredServersResponse(Vec<u8>),      // Response to a list of registered servers
}

impl DroneSend for SimControllerMessage {}
impl Request for SimControllerMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerEvent {
    PacketReceived(u64),                 // Packet id?
    MessageSent(NodeId, NodeId, String), // A message sent by a client (server_id, node_to, message)
    FloodRequestSent,
    PacketSent {
        session_id: u64,
        packet_type: String,
    }, // To recognize the type of the original packet sent without generating a new enum
    PacketDropped {
        session_id: u64,
        packet_type: String,
    }, // To recognize the type of the original packet sent without generating a new enum
    PacketForwarded {
        session_id: u64,
        packet_type: String,
        source: NodeId,
        destination: NodeId,
    }, // Packet forwarded through the simulation controller
}

impl DroneSend for SimControllerEvent {}
impl Request for SimControllerEvent {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerResponseWrapper {
    Message(SimControllerMessage),
    Event(SimControllerEvent),
}

impl DroneSend for SimControllerResponseWrapper {}
