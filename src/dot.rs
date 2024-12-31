use std::fmt::Write;

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
                write!(&mut out, "{} {}", self.nodes[i].string(), op).unwrap();
            } else {
                write!(&mut out, "{}", self.nodes[i].string()).unwrap();
            }
        }
        out
    }
}

fn stringify_attributes(attributes: Vec<(String, String)>) -> String {
    if attributes.is_empty() {
        return String::new();
    }
    String::new()
}
