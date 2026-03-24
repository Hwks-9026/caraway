use std::collections::HashMap;
use std::hash::Hash;

pub struct Node<T> {
    data: T,
    connections: Vec<usize>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, connection: usize) {
        self.connections.push(connection);
    }
}

pub struct Graph<T: Clone + Eq + Hash> {
    nodes: Vec<Node<T>>,
    id_map: HashMap<T, usize>,
    ordering: Option<Vec<T>>
}

impl<T: Clone + Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            id_map: HashMap::new(),
            ordering: None,
        }
    }

    pub fn add_node(&mut self, data: T) {
        self.id_map.insert(data.clone(), self.nodes.len());
        self.nodes.push(Node { data, connections: Vec::new() });
    }

    pub fn add_connection(&mut self, source: T, target: T) -> bool {
        let source_id = match self.id_map.get(&source) {
            Some(id) => {*id},
            None => return false,
        };
        let target_id = match self.id_map.get(&target) {
            Some(id) => {*id},
            None => return false,
        };

        self.nodes[source_id].add_connection(target_id);
        true
    }
    pub fn compute_topological_order(&mut self) {
        let mut topological_order = Vec::new();
        let mut in_degrees = vec![0usize; self.nodes.len()];
        let mut zero_in_degrees = Vec::new();

        for node in &self.nodes {
            for connection in &node.connections {
                in_degrees[*connection] += 1;
            }
        }

        for node in 0usize..self.nodes.len() {
            if in_degrees[node] == 0 {
                zero_in_degrees.push(node);
            }
        }

        loop {
            let node = match zero_in_degrees.pop() {
                Some(node) => node,
                None => break,
            };
            topological_order.push(self.nodes[node].data.clone());

            for connection in self.nodes[node].connections.iter() {
                in_degrees[*connection] -= 1;
                if in_degrees[*connection] == 0 {
                    zero_in_degrees.push(*connection);
                }
            }
        }

        if topological_order.len() == self.nodes.len() {
            self.ordering = Some(topological_order);
        } else {
            self.ordering = None;
        }
    }
    pub fn topological_order(&self) -> Option<Vec<T>> {
        return self.ordering.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_node() {
        let node = Node::new(1);
        assert_eq!(node.data, 1);
        assert_eq!(node.connections.len(), 0);
    }

    #[test]
    fn make_simple_graph() {
        let mut graph = Graph::new();
        graph.add_node(2);
        graph.add_node(1);
        graph.add_connection(1,2);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.id_map.len(), 2);
        assert_eq!(graph.id_map.get(&1).unwrap(), &1);
        assert_eq!(graph.id_map.get(&2).unwrap(), &0);
    }

    #[test]
    fn topological_order_success() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_connection(1,2);
        graph.add_connection(1,3);
        graph.add_connection(2,3);

        graph.compute_topological_order();
        assert_eq!(graph.topological_order(), Some(vec![1,2,3]));

        let mut graph = Graph::new();
        graph.add_node(3);
        graph.add_node(2);
        graph.add_node(1);

        graph.add_connection(1,2);
        graph.add_connection(2,3);
        graph.compute_topological_order();
        assert_eq!(graph.topological_order(), Some(vec![1,2,3]));
    }

    #[test]
    fn topological_order_fail() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_connection(1,2);
        graph.add_connection(2,3);
        graph.add_connection(3,1);

        graph.compute_topological_order();
        assert_eq!(graph.topological_order(), None);
    }
}
