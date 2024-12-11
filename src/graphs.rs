// use petgraph::graph::UnGraph;
// use petgraph::graphmap::{DiGraphMap};
use graphrs::{Edge, Graph, GraphSpecs, Node};
// use petgraph::visit::EdgeCount;

pub fn graphs() {
    // // Directed graph
    // let mut g = DiGraphMap::<&str, i32>::new();
    // g.add_node("a");
    // g.add_node("b");
    // g.add_node("c");
    // g.add_edge("a", "b", 1);
    // g.add_edge("a", "b", 3498);
    // g.add_edge("b", "c", 2);
    // g.add_edge("a", "c", 3);
    // g.add_edge("a", "b", 32);
    // // println!("{:?}", g);
    // println!("{:?}", g.edge_count());
    //
    // // No idea
    // let mut g1 = UnGraph::<&str, i32>::new_undirected();
    // g1.add_node("a");
    // g1.add_node("b");
    // g1.add_node("c");
    // println!("{:?}", g1);

    // normal graph using graphrs crate
    let edges = vec![
        Edge::with_weight("n1", "n2", 1.0),
        Edge::with_weight("n1", "n3", 2.0),
        Edge::with_weight("n1", "n5", 3.0),
        Edge::with_weight("n4", "n5", 4.0),
    ];
    let graph: Graph<&str, ()> =
        Graph::new_from_nodes_and_edges(vec![], edges, GraphSpecs::directed_create_missing())
            .unwrap();

    println!("{:?}", graph.get_all_node_names());

    // directed multi graph using graphrs
    // https://docs.rs/graphrs/latest/graphrs/struct.Graph.html#example
    let specs = GraphSpecs {
        self_loops: false,
        // multi_edges: true,
        ..GraphSpecs::multi_directed()
    };
    let nodes = vec![
        Node::from_name("a"),
        Node::from_name("b"),
        Node::from_name("c"),
    ];

    let edges = vec![
        Edge::with_weight("a", "b", 1.0),
        Edge::with_weight("b", "a", 2.0),
        Edge::with_weight("a", "c", 3.0),
        Edge::with_weight("b", "c", 3.0),
        Edge::with_weight("a", "b", 112.0),
    ];

    let graph = Graph::<&str, ()>::new_from_nodes_and_edges(nodes, edges, specs).unwrap();

    println!("{:?}", graph.get_edges("a", "b"));
}
