use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32) ){
        let (node1, node2, weight) = edge;
        
        // 确保节点存在
        self.add_node(node1);
        self.add_node(node2);
        
        // 获取邻接表的可变引用
        let adj_table = self.adjacency_table_mutable();
        
        // 添加双向边
        adj_table.entry(node1.to_string())
            .and_modify(|edges| edges.push((node2.to_string(), weight)));
        
        adj_table.entry(node2.to_string())
            .and_modify(|edges| edges.push((node1.to_string(), weight)));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        if self.contains(node) {
            false
        } else {
            self.adjacency_table_mutable()
                .insert(node.to_string(), Vec::new());
            true
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        // 默认实现留给具体类型实现
        unimplemented!()
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        
        let expected_edges = [
            (&"a".to_string(), &"b".to_string(), 5),
            (&"b".to_string(), &"a".to_string(), 5),
            (&"c".to_string(), &"a".to_string(), 7),
            (&"a".to_string(), &"c".to_string(), 7),
            (&"b".to_string(), &"c".to_string(), 10),
            (&"c".to_string(), &"b".to_string(), 10),
        ];
        
        for edge in &expected_edges {
            assert!(graph.edges().contains(edge));
        }
    }
}