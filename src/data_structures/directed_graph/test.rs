

#[cfg(test)]
mod test {
    use crate::data_structures::directed_graph::DirectedGraph;

    #[test]
    fn straight_line() {
        let mut foo = DirectedGraph::with_nodes(4);

        foo.add_edge(0, 1, 1);
        foo.add_edge(1, 2, 1);
        foo.add_edge(2, 3, 1);

        assert_eq!(foo.distance(0, 1), 1);
        assert_eq!(foo.distance(0, 2), 2);
        assert_eq!(foo.distance(0, 3), 3);
    }

    #[test]
    fn diamond() {
        let mut foo = DirectedGraph::with_nodes(4);

        foo.add_edge(0, 1, 2);
        foo.add_edge(1, 2, 2);

        foo.add_edge(0, 2, 1);
        foo.add_edge(2, 3, 1);

        assert_eq!(foo.distance(0, 3), 2);
    }
    
    #[test]
    fn cycle() {
        let mut foo = DirectedGraph::with_nodes(4);

        foo.add_edge(0, 2, 5);
        foo.add_edge(1, 0, 1);
        foo.add_edge(2, 1, 1);
        foo.add_edge(2, 3, 1);

        assert_eq!(foo.distance(0, 3), 6);
    }

    #[test]
    fn bigger() {
        let mut foo = DirectedGraph::with_nodes(8);

        foo.add_edge(0, 1, 1);
        foo.add_edge(1, 2, 1);
        foo.add_edge(1, 3, 3);
        foo.add_edge(3, 5, 2);
        foo.add_edge(5, 4, 1);
        foo.add_edge(4, 1, 5);
        foo.add_edge(2, 3, 2);
        foo.add_edge(3, 6, 1);
        foo.add_edge(2, 6, 2);
        foo.add_edge(2, 7, 6);
        foo.add_edge(6, 7, 1);
        foo.add_edge(4, 3, 2);
        foo.add_edge(3, 7, 3);

        assert_eq!(foo.distance(0, 7), 5);
        assert_eq!(foo.distance(5, 7), 5);
    }
}