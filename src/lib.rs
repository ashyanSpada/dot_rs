mod dot;

pub use dot::*;

#[cfg(test)]
mod test {
    #[test]
    fn test_graph() {
        use super::*;
        let node1 = new_node_string("node1");
        let node2 = new_node_string("node2").with_attribute("color", "blue");
        let edge = new_edge(&node1)
            .with_attribute("label", "edge1")
            .add_node(&node2)
            .with_attribute("color", "red")
            .with_attribute("label", "test");
        let graph = new_graph("test", true)
            .add_stmt(&edge)
            .add_stmt(&node1)
            .add_stmt(&node2);
        graph.draw("test.dot");
    }
}
