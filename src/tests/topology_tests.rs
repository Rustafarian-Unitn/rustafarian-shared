#[cfg(test)]
#[allow(unused_imports, unreachable_code, unused_variables)]
pub mod topology_tests {
    use crate::topology::Topology;
    use rand::Rng;
    use wg_2024::network::NodeId;

    #[test]
    fn should_find_best_path() {
        const PDR: f64 = 0.7;
        let mut topology = Topology::new();

        // 11 and 12 are terminal nodes (i.e. client or server), the oder nodes are drones
        update_topology(
            &mut topology,
            vec![1, 2, 3, 4, 5, 6, 11, 12],
            vec![
                (11, 1),
                (1, 2),
                (2, 3),
                (3, 12),
                (11, 4),
                (4, 5),
                (5, 6),
                (6, 12),
            ],
        );

        let route = topology.get_routing_header(11, 12).hops;
        // Simulate packet dropped in the route, updating the history of the nodes
        for node in route.clone() {
            for i in 0..10 {
                topology
                    .update_node_history(&vec![node], rand::thread_rng().gen_range(0.0..1.0) < PDR);
            }
        }

        let new_route = topology.get_routing_header(11, 12).hops;

        // Check that the sender and receiver node are the same
        assert_eq!(route.get(0), new_route.get(0));
        assert_eq!(
            route.get(route.len() - 1),
            new_route.get(new_route.len() - 1)
        );

        // Check that the new route is different from the old one, to account for the pdr
        assert_ne!(route.get(1), new_route.get(1));
        assert_ne!(route.get(2), new_route.get(2));
        assert_ne!(route.get(3), new_route.get(3));
    }

    /// Utility method that updates the current topology of the server,
    /// adding the `nodes` and `edges`
    ///
    /// # Args
    /// * `nodes: Vec<NodeId>` - vector of nodes to add to the topology
    /// * `edges: Vec<(NodeId, NodeId)>` - vector of edges to add to the topology,
    /// both from the first node to the second and vice versa
    pub fn update_topology(
        topology: &mut Topology,
        nodes: Vec<NodeId>,
        edges: Vec<(NodeId, NodeId)>,
    ) {
        // Ad node to node list if not there
        for node in nodes {
            if !topology.nodes().contains(&node) {
                topology.add_node(node);
            }
        }

        // Add edges between the two nodes, if not exists
        for edge in edges {
            if !topology.edges().contains_key(&edge.0)
                || !topology.edges().get(&edge.0).unwrap().contains(&edge.1)
            {
                topology.add_edge(edge.0, edge.1);
                topology.add_edge(edge.1, edge.0);
            }
        }
    }
}
