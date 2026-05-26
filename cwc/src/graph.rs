use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Eq, Clone, Debug)]
pub struct Graph<T: Clone + Eq + Hash> {
    nodes: Vec<T>,
    connections: Vec<HashSet<usize>>,
    id_map: HashMap<T, usize>,
    topological_ordering: Option<Vec<T>>,
}

struct TarjanState {
    sccs: Vec<Vec<usize>>,
    disc: Vec<Option<usize>>,
    low: Vec<Option<usize>>,
    stack: Vec<usize>,
    on_stack: Vec<bool>,
    counter: usize,
}

impl TarjanState {
    fn new(n: usize) -> TarjanState {
        TarjanState {
            sccs: Vec::new(),
            disc: vec![None; n],
            low: vec![None; n],
            stack: Vec::new(),
            on_stack: vec![false; n],
            counter: 0,
        }
    }
}

impl<T: Clone + Eq + Hash> Hash for Graph<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut node_hashes: Vec<u64> = self
            .nodes
            .iter()
            .map(|n| {
                let mut h = DefaultHasher::new();
                n.hash(&mut h);
                h.finish()
            })
            .collect();
        node_hashes.sort();
        node_hashes.hash(state);
    }
}

impl<T: Clone + Eq + Hash> PartialEq for Graph<T> {
    fn eq(&self, other: &Self) -> bool {
        self.comparable_format() == other.comparable_format()
    }
}

impl<T: Clone + Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            connections: Vec::new(),
            id_map: HashMap::new(),
            topological_ordering: None,
        }
    }

    pub fn add_node(&mut self, data: &T) -> bool {
        if self.id_map.contains_key(data) {
            return false;
        }

        if let Some(ordering) = &mut self.topological_ordering {
            ordering.push(data.clone());
        }
        self.id_map.insert(data.clone(), self.nodes.len());
        self.nodes.push(data.clone());
        self.connections.push(HashSet::new());

        true
    }

    pub fn add_connection(&mut self, source: &T, target: &T) -> bool {
        let source_id = match self.id_map.get(source) {
            Some(id) => *id,
            None => return false,
        };
        let target_id = match self.id_map.get(target) {
            Some(id) => *id,
            None => return false,
        };

        if self.connections[source_id].contains(&target_id) {
            return false;
        }

        self.connections[source_id].insert(target_id);
        self.topological_ordering = None;

        true
    }

    fn comparable_format(&self) -> HashMap<&T, HashSet<&T>> {
        let mut to_return = HashMap::new();

        for (node, source_id) in self.id_map.iter() {
            let mut node_connections = HashSet::new();
            for connection in self.connections[*source_id].iter() {
                node_connections.insert(&self.nodes[*connection]);
            }

            to_return.insert(node, node_connections);
        }

        to_return
    }

    pub fn topological_order(&mut self) -> Option<Vec<T>> {
        if let Some(ordering) = self.topological_ordering.clone() {
            return Some(ordering);
        }

        let mut topological_order = Vec::new();
        let mut in_degrees = vec![0usize; self.nodes.len()];
        let mut zero_in_degrees = Vec::new();

        for (i, _) in self.nodes.iter().enumerate() {
            for connection in &self.connections[i] {
                in_degrees[*connection] += 1;
            }
        }

        for (i, _) in self.nodes.iter().enumerate() {
            if in_degrees[i] == 0 {
                zero_in_degrees.push(i);
            }
        }

        while let Some(node) = zero_in_degrees.pop() {
            topological_order.push(self.nodes[node].clone());

            for connection in self.connections[node].iter() {
                in_degrees[*connection] -= 1;
                if in_degrees[*connection] == 0 {
                    zero_in_degrees.push(*connection);
                }
            }
        }

        if topological_order.len() == self.nodes.len() {
            self.topological_ordering = Some(topological_order);
        } else {
            self.topological_ordering = None;
        }

        self.topological_ordering.clone()
    }

    fn tarjan(&self, state: &mut TarjanState, node: usize) {
        state.disc[node] = Some(state.counter);
        state.low[node] = Some(state.counter);
        state.counter += 1;

        state.stack.push(node);
        state.on_stack[node] = true;

        for &connection in self.connections[node].iter() {
            if state.disc[connection].is_none() {
                self.tarjan(state, connection);
                state.low[node] = min(state.low[node], state.low[connection]);
            } else if state.on_stack[connection] {
                state.low[node] = min(state.low[node], state.disc[connection]);
            }
        }

        if state.low[node] == state.disc[node] {
            let mut scc = Vec::new();
            loop {
                let popped_node = state.stack.pop().unwrap();
                state.on_stack[popped_node] = false;
                scc.push(popped_node);
                if node == popped_node {
                    break;
                }
            }
            state.sccs.push(scc);
        }
    }

    fn scc_dag(&self) -> Graph<Graph<T>> {
        let mut state = TarjanState::new(self.nodes.len());

        for (i, _) in self.nodes.iter().enumerate() {
            if state.disc[i].is_some() {
                continue;
            }
            self.tarjan(&mut state, i);
        }

        let mut scc_graphs = vec![Graph::new(); state.sccs.len()];
        let mut scc_number = vec![0; self.nodes.len()];

        for (num, scc) in state.sccs.iter().enumerate() {
            for &node in scc {
                scc_number[node] = num;
                scc_graphs[num].add_node(&self.nodes[node]);
            }
        }

        let mut dag_connections = vec![HashSet::new(); state.sccs.len()];

        for (node, edges) in self.connections.iter().enumerate() {
            let source_scc = scc_number[node];
            for &edge in edges {
                let target_scc = scc_number[edge];
                if source_scc == target_scc {
                    scc_graphs[source_scc].add_connection(&self.nodes[node], &self.nodes[edge]);
                } else {
                    dag_connections[source_scc].insert(target_scc);
                }
            }
        }

        let mut dag = Graph::new();

        for scc in scc_graphs.iter() {
            dag.add_node(scc);
        }
        for (i, connections) in dag_connections.iter().enumerate() {
            for &connection in connections {
                dag.add_connection(&scc_graphs[i], &scc_graphs[connection]);
            }
        }

        dag
    }

    pub fn list_cycles(&self) -> Option<Vec<Vec<T>>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_simple_graph() {
        let mut graph = Graph::new();
        graph.add_node(&2);
        graph.add_node(&1);
        graph.add_connection(&1, &2);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.id_map.len(), 2);
        assert_eq!(graph.id_map.get(&1).unwrap(), &1);
        assert_eq!(graph.id_map.get(&2).unwrap(), &0);
    }

    #[test]
    fn graph_equal_simple() {
        let mut graph1 = Graph::new();
        let mut graph2 = Graph::new();

        graph1.add_node(&1);
        graph2.add_node(&1);

        graph1.add_node(&2);
        graph2.add_node(&2);

        graph1.add_connection(&1, &2);
        graph2.add_connection(&1, &2);

        assert_eq!(graph1, graph2);
    }

    #[test]
    fn graph_equal_complex() {
        let mut graph1 = Graph::new();
        let mut graph2 = Graph::new();

        graph1.add_node(&1);
        graph1.add_node(&2);

        graph2.add_node(&2);
        graph2.add_node(&1);

        graph1.add_connection(&1, &2);
        graph1.add_connection(&2, &1);

        graph2.add_connection(&2, &1);
        graph2.add_connection(&1, &2);
    }

    #[test]
    fn graph_unqual() {
        let mut graph1 = Graph::new();
        let mut graph2 = Graph::new();

        graph1.add_node(&1);
        graph2.add_node(&1);

        graph1.add_node(&2);
        graph2.add_node(&2);

        graph1.add_connection(&1, &2);
        graph2.add_connection(&2, &1);

        assert_ne!(graph1, graph2);
    }

    #[test]
    fn topological_order_success() {
        let mut graph = Graph::new();
        graph.add_node(&1);
        graph.add_node(&2);
        graph.add_node(&3);

        graph.add_connection(&1, &2);
        graph.add_connection(&1, &3);
        graph.add_connection(&2, &3);

        assert_eq!(graph.topological_order(), Some(vec![1, 2, 3]));

        let mut graph = Graph::new();
        graph.add_node(&3);
        graph.add_node(&2);
        graph.add_node(&1);

        graph.add_connection(&1, &2);
        graph.add_connection(&2, &3);
        assert_eq!(graph.topological_order(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn topological_order_fail() {
        let mut graph = Graph::new();
        graph.add_node(&1);
        graph.add_node(&2);
        graph.add_node(&3);
        graph.add_connection(&1, &2);
        graph.add_connection(&2, &3);
        graph.add_connection(&3, &1);

        assert_eq!(graph.topological_order(), None);
    }

    #[test]
    fn scc_dag_simple() {
        let mut parent = Graph::new();
        parent.add_node(&1);
        parent.add_node(&2);
        parent.add_node(&3);
        parent.add_connection(&1, &2);
        parent.add_connection(&2, &1);
        parent.add_connection(&2, &3);

        let dag = parent.scc_dag();

        let mut scc1 = Graph::new();
        scc1.add_node(&1);
        scc1.add_node(&2);
        scc1.add_connection(&1, &2);
        scc1.add_connection(&2, &1);

        let mut scc2 = Graph::new();
        scc2.add_node(&3);

        let mut target_dag = Graph::new();

        target_dag.add_node(&scc1);
        target_dag.add_node(&scc2);
        target_dag.add_connection(&scc1, &scc2);

        assert_eq!(dag, target_dag);
    }

    #[test]
    fn list_cycles_success_one_scc() {
        let mut graph = Graph::new();
        graph.add_node(&1);
        graph.add_node(&2);
        graph.add_node(&3);
        graph.add_connection(&1, &2);
        graph.add_connection(&2, &1);
        graph.add_connection(&1, &3);
        graph.add_connection(&3, &2);

        // Unique cycles
        // 1 -> 2 -> 1
        // 1 -> 3 -> 2 -> 1

        assert_ne!(graph.list_cycles(), None);
        assert_eq!(graph.list_cycles().unwrap().len(), 2);
    }
}
