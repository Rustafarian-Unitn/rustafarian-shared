use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use wg_2024::network::{NodeId, SourceRoutingHeader};

/// A simple graph representation of the network topology
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topology {
    nodes: Vec<NodeId>,                      // The list of nodes in the topology
    edges: HashMap<NodeId, HashSet<NodeId>>, // All the connections between nodes.
    labels: HashMap<NodeId, String>,         // The labels of the nodes
    node_types: HashMap<NodeId, String>,     // The types of the nodes
}

impl Default for Topology {
    fn default() -> Self {
        Self::new()
    }
}

impl Topology {
    /// Create a new empty topology
    pub fn new() -> Self {
        Topology {
            nodes: Vec::new(),
            edges: HashMap::new(),
            labels: HashMap::new(),
            node_types: HashMap::new(),
        }
    }

    /// Add a new node to the topology (NodeId: u8)
    pub fn add_node(&mut self, node: NodeId) {
        self.nodes.push(node);
        self.edges.insert(node, HashSet::new());
    }

    /// Add a new edge between two nodes
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.edges.entry(from).or_default().insert(to);
        self.edges.entry(to).or_default().insert(from);
    }

    /// Get the neighbors of a node
    pub fn neighbors(&self, node_id: NodeId) -> Vec<NodeId> {
        self.edges.get(&node_id).unwrap_or(&HashSet::new()).iter().copied().collect()
    }

    /// Clear the topology
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    /// Get the nodes of the topology
    pub fn nodes(&self) -> &Vec<NodeId> {
        &self.nodes
    }

    /// Get the edges of the topology
    pub fn edges(&self) -> &HashMap<NodeId, HashSet<NodeId>> {
        &self.edges
    }

    pub fn get_routing_header(
        &self,
        client_id: NodeId,
        server_id: NodeId,
    ) -> wg_2024::network::SourceRoutingHeader {
        let mut header = SourceRoutingHeader::empty_route();
        header.hops = compute_route(self, client_id, server_id);
        header.hop_index = 1;
        header
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        self.nodes.retain(|&x| x != node_id);
        self.edges.remove(&node_id);
        for (_, neighbors) in self.edges.iter_mut() {
            neighbors.retain(|&x| x != node_id);
        }
    }

    pub fn get_label(&self, node_id: NodeId) -> Option<&String> {
        self.labels.get(&node_id)
    }

    pub fn set_label(&mut self, node_id: NodeId, label: String) {
        self.labels.insert(node_id, label);
    }

    pub fn get_node_type(&self, node_id: NodeId) -> Option<&String> {
        self.node_types.get(&node_id)
    }

    pub fn set_node_type(&mut self, node_id: NodeId, node_type: String) {
        self.node_types.insert(node_id, node_type);
    }
}

// BFS search between a starting node and a destination
pub fn compute_route(
    topology: &Topology,
    source_id: NodeId,
    destination_id: NodeId,
) -> Vec<NodeId> {
    let mut route = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(source_id);
    visited.insert(source_id);
    let mut parent = HashMap::new();
    while !queue.is_empty() {
        let current_node = queue.pop_front().unwrap();
        if current_node == destination_id {
            let mut node = destination_id;
            while node != source_id {
                route.push(node);
                node = parent[&node];
            }
            route.push(source_id);
            route.reverse();
            return route;
        }
        for neighbor in topology.neighbors(current_node) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent.insert(neighbor, current_node);
                queue.push_back(neighbor);
            }
        }
    }
    route
}
