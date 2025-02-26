use std::collections::HashMap;

pub struct Edge {
    from_node: u32,
    to_node: u32,
    weight: f32,
}

impl Edge {
    pub fn new(from_node: u32, to_node: u32, weight: f32) -> Self {
        Self {
            from_node,
            to_node,
            weight,
        }
    }
}

pub struct Node<T> {
    pub idx: u32,
    pub edges: HashMap<u32, Edge>,
    pub label: Option<T>,
}

impl<T> Node<T> {
    /// Creates a new node without label.
    /// # Example
    /// ```
    /// use graphs::Node;
    ///
    /// let node: Node<()> = Node::new(10);
    /// assert_eq!(node.idx, 10);
    /// ```
    ///
    pub fn new(idx: u32) -> Self {
        Self {
            idx,
            edges: HashMap::new(),
            label: None,
        }
    }
    /// Creates a new node with a label.
    /// You can define your own label type by implementing the `Debug` trait for
    /// tye type.
    ///
    ///# Example
    /// ```
    /// use graphs::Node;
    ///
    /// #[derive(Debug)]
    /// struct Cart {
    ///     amount: f64,
    ///     name: String,
    /// }
    ///
    ///
    /// let label = Cart { amount: 510.50, name: String::from("Alex Jones")};
    /// let labelled_node = Node::with_label(0, label);
    /// assert_eq!(labelled_node.idx, 0);
    /// ```
    pub fn with_label(idx: u32, label: T) -> Self {
        Self {
            idx,
            edges: HashMap::new(),
            label: Some(label),
        }
    }
    /// Gets the number of edges.
    pub fn number_of_edges(&self) -> usize {
        self.edges.len()
    }
    /// Returns a reference to the edge object that connects this node with the
    /// `neighbor` node.
    pub fn get_edge(&self, neighbor: u32) -> Option<&Edge> {
        self.edges.get(&neighbor)
    }
    /// Connects an edge between this node and the `neighbor` node.
    /// If the source node already has an edge to a neighbor node, 
    /// it updates the edge data and returns the old edge value. 
    /// Otherwise returns `None`.
    pub fn add_edge(&mut self, neighbor: u32, weight: f32) -> Option<Edge> {
        let new_edge = Edge::new(self.idx, neighbor, weight);
        eprintln!("{:?}", new_edge.from_node);
        eprintln!("{:?}", new_edge.to_node);
        eprintln!("{:?}", new_edge.weight);
        self.edges.insert(neighbor, new_edge)
    }
    /// Removes an edge between two nodes.
    /// Returns the index of the neighbor node and the edge value if the edge existed between
    /// the two nodes.
    pub fn remove_edge(&mut self, neighbor: u32) -> Option<(u32, Edge)> {
        self.edges.remove_entry(&neighbor)
    }
}

pub struct Graph<T> {
    pub capacity: usize,
    pub nodes: Vec<Node<T>>,
    pub undirected: bool,
}

impl<T> Graph<T> {
    /// Creates a new grpah with the given capacity and directional type.
    pub fn new(capacity: usize, undirected: bool) -> Self {
        Self {
            capacity,
            nodes: Vec::with_capacity(capacity),
            undirected,
        }
    }
    /// Inserts an edge between two nodes in the graph.
    pub fn insert_edge(&mut self, from: u32, to: u32, weight: f32) -> Option<Edge> {
        if let Some(src_node_idx) = self.nodes.iter().position(|n| n.idx == from) {
            self.nodes[src_node_idx].add_edge(to, weight)
        } else {
            //TODO: None is returned for two success cases, so should do something 
            //like make an error enum when None is returned for nonexistent 
            //nodes like here
            None
        }
        //let src_node_idx = self.nodes.partition_point(|node| node.idx == from) - 1;
    }
    pub fn insert_node(&mut self, node: Node<T>) -> bool {
        if self.nodes.len() == self.capacity {
            false
        } else {
            self.nodes.push(node);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_nodes_and_edges() {
        let mut node_10 = Node::<()>::new(10);
        let node_20 = Node::<()>::new(20);
        let node_30 = Node::<()>::new(30);
        let node_40 = Node::<()>::new(40);

        node_10.add_edge(40, 2024.0);
        node_10.add_edge(20, 24.33);
        node_10.add_edge(30, 8902.0);

        assert!(node_10.number_of_edges() == 3);
        let edge = node_10.get_edge(20).unwrap();
        assert_eq!(edge.from_node, 10);
        assert_eq!(edge.to_node, 20);
        assert_eq!(edge.weight, 24.33);
    }

    #[test]
    fn test_creating_labelled_nodes() {
        let n10_label = String::from("Furniture");
        let node_10: Node<String> = Node::with_label(10, n10_label);
        assert_eq!(node_10.label.unwrap(), String::from("Furniture"));
    }

    #[test]
    fn test_creating_graph_objects_with_non_labelled_nodes() {
        let graph: Graph<()> = Graph::new(5, false);
        assert_eq!(graph.capacity, 5);
    }

    #[test]
    fn test_creating_graph_objects_with_labelled_nodes() {
        let mut graph: Graph<String> = Graph::new(5, false);
        assert_eq!(graph.capacity, 5);

        let node_20: Node<String> = Node::with_label(20, String::from("Furniture"));
        let node_30: Node<String> = Node::with_label(30, String::from("Laptop"));
        let node_40: Node<String> = Node::with_label(40, String::from("Clock"));

        graph.insert_node(node_20);
        graph.insert_node(node_30);
        graph.insert_node(node_40);
        
        let edge = graph.insert_edge(40, 20, 1012.10);
        assert!(edge.is_none());
        assert_eq!(graph.nodes.len(), 3);
    }
}
