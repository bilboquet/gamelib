pub mod graph;
use graph::Graph;

fn main() {
    let mut graph = Graph::new();

    for _ in 0..4 {
        graph.vertices.push(graph::vertex::Vertex {
            id: graph.next_vertex_id(),
            degree: 0u8,
        });
    }

    dbg!(&graph);
    graph.render();
}

