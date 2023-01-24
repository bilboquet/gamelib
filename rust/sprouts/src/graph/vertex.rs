pub type VertexId = usize;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub(crate) id: VertexId,
    pub(crate) degree: u8,
}

impl Vertex {
    pub fn new(id: VertexId, degree: u8) -> Self {
        Self { id, degree }
    }
}
