use serde::{Deserialize, Serialize};
use wg_2024::network::NodeId;

use crate::messages::general_messages::{DroneSend, Request};

/**
 * Command that can be sent from the simulation controller to the clients
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimControllerCommand {
    SendMessage(String, NodeId, NodeId), // Send message to a server, the first id is the server, the second the destination client
    Register(NodeId),            // Register a client to a server
    ClientList(NodeId),          // Get the list of available clients from a server
    FloodRequest,                // Send a flood request
    Topology,                    // Get the topology of the network
}

impl DroneSend for SimControllerCommand {}
impl Request for SimControllerCommand {}