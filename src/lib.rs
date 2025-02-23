use std::{collections::HashMap, fmt::Debug};

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

pub struct Node<T: Debug> {
    pub idx: u32,
    pub edges: HashMap<u32, Edge>,
    pub label: Option<T>,
}

impl<T: Debug> Node<T> {
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
    /// Returns the new edge object.
    pub fn add_edge(&mut self, neighbor: u32, weight: f32) -> Option<Edge> {
        let new_edge = Edge::new(self.idx, neighbor, weight);
        self.edges.insert(neighbor, new_edge)
    }
    /// Removes an edge between two nodes.
    /// Returns the index of the neighbor node and the edge value if the edge existed between
    /// the two nodes.
    pub fn remove_edge(&mut self, neighbor: u32) -> Option<(u32, Edge)> {
        self.edges.remove_entry(&neighbor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_edge_list() {
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
}
