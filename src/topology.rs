use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use wg_2024::network::{NodeId, SourceRoutingHeader};

/// History of a drone, recording the total number of packet sent and the number of packet dropped
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NodePacketHistory {
    pub packets_sent: u64,
    pub packets_dropped: u64,
}

/// A simple graph representation of the network topology
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topology {
    nodes: Vec<NodeId>,                      // The list of nodes in the topology
    edges: HashMap<NodeId, HashSet<NodeId>>, // All the connections between nodes.
    labels: HashMap<NodeId, String>,         // The labels of the nodes
    node_types: HashMap<NodeId, String>,     // The types of the nodes

    // PDR Mapping
    node_histories: HashMap<NodeId, NodePacketHistory>,
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
            node_histories: HashMap::new(),
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
        self.edges
            .get(&node_id)
            .unwrap_or(&HashSet::new())
            .iter()
            .copied()
            .collect()
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
        &mut self,
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

    /// Function that removed the edges between two node, both from node1 to node2 and vice versa
    pub fn remove_edges(&mut self, node1: NodeId, node2: NodeId) {
        for (&id, neighbors) in self.edges.iter_mut() {
            if id == node1 {
                neighbors.retain(|&id| id != node2);
            } else if id == node2 {
                neighbors.retain(|&id| id != node1);
            }
        }
    }

    /// Function that updates the history of a list of nodes, based on the drooped flag.
    /// Should only be called for MsgFragment, since they are the only droppable packets
    ///
    /// # Args
    /// * `node_id: Vec<NodeId>` - vector of nodes to update
    /// * `dropped: bool` - if `true` then will increase the packets_dropped, else the packets_sent
    pub fn update_node_history(&mut self, node_ids: &Vec<NodeId>, dropped: bool) {
        for id in node_ids {
            let history = self.node_histories.entry(*id).or_default();
            if dropped {
                history.packets_dropped += 1;
            } else {
                history.packets_sent += 1;
            }
        }
    }

    /// Function that returns the estimated PDR, from 0 to 100, based on the history of the node.
    pub fn pdr_for_node(&mut self, node_id: NodeId) -> u64 {
        let history = self.node_histories.entry(node_id).or_default();

        if history.packets_sent > 0 {
            ((history.packets_dropped as f64 / history.packets_sent as f64) * 100f64) as u64
        } else {
            0
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

    pub fn get_node_types(&self) -> &HashMap<NodeId, String> {
        &self.node_types
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

#[derive(Eq, PartialEq, Debug)]
/// Used to store distance information for the node, and sort them in the BinaryHeap
struct Node {
    id: NodeId,
    distance: u64
}

// Invert ordering for binary heap, by default it prioritize higher values
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Compute a route between two nodes, using an adaptation of the Dijkstra algorithm, where the
/// distance between the nodes is found using the PDR of the node
pub fn compute_route_dijkstra(
    topology: &mut Topology,
    source_id: NodeId,
    destination_id: NodeId,
) -> Vec<NodeId> {
    let mut route = Vec::new(); // Final route
    let mut visited = HashSet::new(); // Node already visited
    let mut queue = BinaryHeap::new(); // Used to prioritize nodes based on PDR

    // "Distances" to each node, it is based on the PDR for each node
    let mut distances = HashMap::new();
    let mut parent = HashMap::new();

    // Initiate the source with distance 0, since it is the starting point
    distances.insert(source_id, 0);
    queue.push(Node { id: source_id, distance: 0 });

    while let Some(node) = queue.pop() {
        // If destination is reached, then reconstruct the path, based on the parents nodes and
        // reversing it at the end
        if node.id == destination_id {
            let mut node = destination_id;
            while node != source_id {
                route.push(node);
                node = parent[&node];
            }
            route.push(source_id);
            route.reverse();
            return route;
        }

        // If the node has already been visited, then ignore it
        if !visited.insert(node.id) {
            continue;
        }

        for neighbor in topology.neighbors(node.id) {
            // Skip neighbors that are not of type "Drone" unless they are the destination
            if topology.get_node_type(neighbor).map_or(false, |t| t != "drone") && neighbor != destination_id {
                continue;
            }
            // For every neighbour of the current node, find the distance (cumulative)
            // from the source to the node, based on the PDR
            let drop_rate = topology.pdr_for_node(neighbor);
            let new_distance = node.distance + drop_rate;

            // If the distance is less then the one that was already found, then insert
            // this as the new distance, and update the parent map
            if new_distance < *distances.get(&neighbor).unwrap_or(&u64::MAX) {
                distances.insert(neighbor, new_distance);
                parent.insert(neighbor, node.id);
                queue.push(Node{ id: neighbor, distance: new_distance });
            }
        }
    }
    route
}
