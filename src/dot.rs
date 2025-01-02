use std::{fmt::Write, fs::File, io::Write as io_write};

pub trait Node {
    fn string(&self) -> String;
}

pub struct NodeString {
    pub val: String,
}

pub fn new_node_string(val: &str) -> NodeString {
    return NodeString {
        val: val.to_string(),
    };
}

impl Node for NodeString {
    fn string(&self) -> String {
        self.val.clone()
    }
}

pub struct Edge<'a> {
    pub nodes: Vec<&'a dyn Node>,
    pub attributes: Vec<(String, String)>,
}

pub fn new_edge<'a>(lhs: &'a dyn Node) -> Edge<'a> {
    Edge {
        nodes: vec![lhs],
        attributes: Vec::new(),
    }
}

pub trait Stmt {
    fn string(&self, _: bool) -> String;
    fn stmt(&self);
}

impl<'a> Edge<'a> {
    pub fn with_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.attributes.push((key.to_string(), value.to_string()));
        self
    }
}

impl<'a> Stmt for Edge<'a> {
    fn stmt(&self) {}
    fn string(&self, directed: bool) -> String {
        let mut out = String::new();
        let op = if directed { "->" } else { "--" };
        for i in 0..self.nodes.len() {
            if i != self.nodes.len() - 1 {
                write!(&mut out, "{} {} ", self.nodes[i].string(), op).unwrap();
            } else {
                write!(&mut out, "{}", self.nodes[i].string()).unwrap();
            }
        }
        out + &stringify_attributes(&self.attributes)
    }
}

pub struct Graph<'a> {
    pub directed: bool,
    pub name: String,
    pub stmts: Vec<&'a dyn Stmt>,
    pub node_attributes: Vec<(String, String)>,
    pub edge_attributes: Vec<(String, String)>,
    pub graph_attributes: Vec<(String, String)>,
}

impl<'a> Graph<'a> {
    pub fn with_node_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.node_attributes
            .push((key.to_string(), value.to_string()));
        self
    }
    pub fn with_edge_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.edge_attributes
            .push((key.to_string(), value.to_string()));
        self
    }
    pub fn with_graph_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.graph_attributes
            .push((key.to_string(), value.to_string()));
        self
    }
    pub fn add_stmt(&'a mut self, stmt: &'a dyn Stmt) -> &'a Self {
        self.stmts.push(stmt);
        self
    }
    pub fn draw(&'a self, path_name: &str) {
        let s = self.string();
        let mut file = File::create(path_name).unwrap();
        file.write(s.as_bytes()).unwrap();
    }
}

impl<'a> Graph<'a> {
    fn string(&self) -> String {
        let mut out = String::new();
        let prefix = if self.directed { "digraph" } else { "graph" };
        write!(out, "{} {} \n {{ \n", prefix, self.name).unwrap();
        write!(
            out,
            "{};\n{};\n;{};\n",
            stringify_attributes_with_prefix("graph", &self.graph_attributes),
            stringify_attributes_with_prefix("edge", &self.edge_attributes),
            stringify_attributes_with_prefix("node", &self.node_attributes),
        )
        .unwrap();
        for stmt in &self.stmts {
            write!(out, "{};\n", stmt.string(self.directed)).unwrap();
        }
        out + "}"
    }
}

pub fn new_graph(name: &str, directed: bool) -> Graph {
    Graph {
        name: name.to_string(),
        directed: directed,
        stmts: Vec::new(),
        node_attributes: Vec::new(),
        edge_attributes: Vec::new(),
        graph_attributes: Vec::new(),
    }
}

pub struct SubGraph<'a> {
    pub directed: bool,
    pub name: String,
    pub stmts: Vec<&'a dyn Stmt>,
    pub node_attributes: Vec<(String, String)>,
    pub edge_attributes: Vec<(String, String)>,
    pub graph_attributes: Vec<(String, String)>,
}

impl<'a> SubGraph<'a> {
    pub fn with_node_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.node_attributes
            .push((key.to_string(), value.to_string()));
        self
    }
    pub fn with_edge_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.edge_attributes
            .push((key.to_string(), value.to_string()));
        self
    }
    pub fn with_graph_attribute(&'a mut self, key: &str, value: &str) -> &'a Self {
        self.graph_attributes
            .push((key.to_string(), value.to_string()));
        self
    }
    pub fn add_stmt(&'a mut self, stmt: &'a dyn Stmt) -> &'a Self {
        self.stmts.push(stmt);
        self
    }
}

impl<'a> Stmt for SubGraph<'a> {
    fn stmt(&self) {}
    fn string(&self, directed: bool) -> String {
        let mut out = String::new();
        let prefix = "subgraph";
        write!(out, "{} {} \n {{ \n", prefix, self.name).unwrap();
        write!(
            out,
            "{};\n{};\n;{};\n",
            stringify_attributes_with_prefix("graph", &self.graph_attributes),
            stringify_attributes_with_prefix("edge", &self.edge_attributes),
            stringify_attributes_with_prefix("node", &self.node_attributes),
        )
        .unwrap();
        for stmt in &self.stmts {
            write!(out, "{};\n", stmt.string(directed)).unwrap();
        }
        out + "}"
    }
}

pub fn new_subgraph(name: &str, directed: bool) -> SubGraph {
    SubGraph {
        name: name.to_string(),
        directed: directed,
        stmts: Vec::new(),
        node_attributes: Vec::new(),
        edge_attributes: Vec::new(),
        graph_attributes: Vec::new(),
    }
}

fn stringify_attributes_with_prefix(prefix: &str, attributes: &Vec<(String, String)>) -> String {
    if attributes.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    write!(
        out,
        "{} {}",
        prefix.to_string(),
        stringify_attributes(attributes)
    )
    .unwrap();
    out
}

fn stringify_attributes(attributes: &Vec<(String, String)>) -> String {
    if attributes.is_empty() {
        return String::new();
    }
    let mut out = "[".to_string();
    for i in 0..attributes.len() {
        write!(out, "{} = \"{}\"", attributes[i].0, attributes[i].1).unwrap();
    }
    out + "]"
}

mod test {
    use super::*;
    #[test]
    fn test_graph() {
        let node1 = new_node_string("node1");
        let node2 = new_node_string("node2");
        let mut edge = new_edge(&node1);
        edge.with_attribute("label", "edge1");
        let mut graph = new_graph("test", true);
        graph.add_stmt(&edge);
        graph.draw("test.dot");
    }
}
