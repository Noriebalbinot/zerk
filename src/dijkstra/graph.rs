use std::collections::HashSet;

#[derive(Default, PartialEq, Eq, Hash)]
pub struct Vertex {
    name: &'static str,
}

#[derive(Default)]
pub struct Edge {
    weigh: u32,
    from: Vertex,
    to: Vertex,
}

#[derive(Default)]
pub struct Graph {
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph::default()
    }
    pub fn add_vertex(&mut self, name: &'static str) {
        self.vertices.insert(Vertex { name });
    }
    pub fn get_vertex(&self, name: &'static str) -> Option<&Vertex> {
        self.vertices.get(&Vertex { name })
    }
}
