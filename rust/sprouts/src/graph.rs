use std::collections::BTreeMap;

pub mod cycle;
pub mod vertex;
use crate::graph::vertex::{Vertex, VertexId};

#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: BTreeMap<VertexId, VertexId>,
    pub faces: Vec<cycle::Cycle>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: vec![],
            edges: BTreeMap::new(),
            faces: vec![],
        }
    }

    pub fn next_vertex_id(&self) -> VertexId {
        if self.vertices.is_empty() {
            0usize
        } else {
            self.vertices.last().unwrap().id + 1
        }
    }

    pub fn render(&self) {
        use petgraph::{
            dot::{Config, Dot},
            Graph, Undirected,
        };
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::process::Command;

        let mut pgraph = Graph::<&Vertex, (), Undirected>::new_undirected();

        for v in self.vertices.iter() {
            pgraph.add_node(&v);
        }
        for e in self.edges.iter() {
pgraph.f
            // graph.add_node()
        }
        pgraph.add_edge(n1, n2, ());
        pgraph.add_edge(n2, n3, 0);
        pgraph.add_edge(n3, n4, 0);
        pgraph.add_edge(n4, n1, 0);

        let file = File::create("graph.dot").unwrap();
        let mut writer = BufWriter::new(file);

        let dot = Dot::with_config(&pgraph, &[Config::EdgeNoLabel]);
        writer.write(dot.to_string().as_bytes()).unwrap();

        let output = Command::new("dot")
            .arg("-Tpng")
            .arg("graph.dot")
            .arg("-o")
            .arg("graph.png")
            .spawn()
            .expect("échec de l'exécution de la commande");
    }
}
