#![crate_name = "graph_algos"]
#![crate_type = "lib"]
#![crate_type = "rlib"]

pub mod traits;
pub mod adjacency_list;
pub mod adjacency_matrix;

pub use adjacency_list::AdjacencyList;
pub use adjacency_matrix::AdjacencyMatrix;

pub use traits::Graph as GraphTrait;
pub use traits::AdjacencyGraph;
pub use traits::IncidenceGraph;
pub use traits::BidirectionalGraph;
pub use traits::VertexListGraph;
pub use traits::EdgeListGraph;
pub use traits::MutableGraph;
pub use traits::AdjacencyMatrixGraph;
