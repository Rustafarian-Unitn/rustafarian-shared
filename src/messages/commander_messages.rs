use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

use crate::messages::general_messages::{DroneSend, Request};

/**
 * Command that can be sent from the simulation controller to the (chat) clients
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerChatCommand {
    SendMessage(String, NodeId, NodeId), // Send message to a server, the first id is the server, the second the destination client
    Register(NodeId),            // Register a client to a server
    ClientList(NodeId),          // Get the list of available clients from a server
    FloodRequest,                // Send a flood request
    Topology,                    // Get the topology of the network
}

impl DroneSend for SimControllerChatCommand {}
impl Request for SimControllerChatCommand {}

/**
 * Messages that can be sent from the clients to the simulation controller
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerMessage {
    FloodResponse(NodeId), // Response to a flood request
    TopologyResponse(NodeId, Vec<NodeId>), // Response to a topology request
    ClientListResponse(NodeId, Vec<NodeId>), // The client list associated to a server, as the client knows it
    MessageReceived(NodeId, NodeId, String), // A message received by a client (server_id, node_from, message)
}

impl DroneSend for SimControllerMessage {}
impl Request for SimControllerMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerEvent {
    PacketReceived(u64), // Packet id?
    MessageSent(NodeId, NodeId, String), // A message sent by a client (server_id, node_to, message)
    FloodRequestSent,
    PacketSent(u64), // Maybe?
}

impl DroneSend for SimControllerEvent {}
impl Request for SimControllerEvent {}

pub enum SimControllerResponseWrapper {
    Message(SimControllerMessage),
    Event(SimControllerEvent),
}