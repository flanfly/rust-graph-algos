use std::collections::HashMap;
use std::slice::Iter;

#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
pub struct GraphVertexDescriptor(pub usize);
#[derive(PartialEq,Eq,Hash,Copy,Clone,Debug)]
pub struct GraphEdgeDescriptor(pub usize);

pub struct Graph<N,E> {
    vertex_labels:  HashMap<GraphVertexDescriptor,N>,
    edge_labels:    HashMap<GraphEdgeDescriptor,E>,
    out_edges:      HashMap<GraphVertexDescriptor,Vec<GraphEdgeDescriptor>>,
    in_edges:       HashMap<GraphVertexDescriptor,Vec<GraphEdgeDescriptor>>,
    edges:          HashMap<GraphEdgeDescriptor,(GraphVertexDescriptor,GraphVertexDescriptor)>,
    next_edge:      GraphEdgeDescriptor,
    next_vertex:    GraphVertexDescriptor
}

pub struct GraphAdjacency {
    adj: Box<Vec<GraphVertexDescriptor>>
}

pub trait Digraph<'a,V,E> {
    type Vertex;
    type Edge;
    type Vertices: Iterator<Item=&'a Self::Vertex>;
    type Edges: Iterator<Item=&'a Self::Edge>;
    type Incidence: Iterator<Item=&'a Self::Edge>;
    type Adjacency: Iterator<Item=Self::Vertex>;

    fn new() -> Self;
    fn add_vertex(&mut self,V) -> Self::Vertex;
    fn add_edge(&mut self,E,Self::Vertex,Self::Vertex) -> Option<Self::Edge>;
    fn remove_vertex<'t>(&'t mut self,&Self::Vertex);
    fn remove_edge(&mut self,&Self::Edge);

    fn num_vertices(&self) -> usize;
    fn num_edges(&self) -> usize;
    fn edge_label(&self,&Self::Edge) -> Option<&E>;
    fn edge_label_mut(&mut self,&Self::Edge) -> Option<&mut E>;
    fn vertex_label(&self,&Self::Vertex) -> Option<&V>;
    fn vertex_label_mut(&mut self,&Self::Vertex) -> Option<&mut V>;
    fn source(&self,&Self::Edge) -> Self::Vertex;
    fn target(&self,&Self::Edge) -> Self::Vertex;

    fn vertices(&'a self) -> Self::Vertices;
    fn edges(&'a self) -> Self::Edges;

    fn in_degree(&'a self, &Self::Vertex) -> usize;
    fn out_degree(&'a self, &Self::Vertex) -> usize;
    fn degree(&'a self, &Self::Vertex) -> usize;
    fn out_edges(&'a self, &Self::Vertex) -> Self::Incidence;
    fn in_edges(&'a self, &Self::Vertex) -> Self::Incidence;
    fn adjacent_vertices(&'a self, &Self::Vertex) -> Self::Adjacency;
}

impl Iterator for GraphAdjacency {
    type Item = GraphVertexDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        return self.adj.pop();
    }
}

impl<'a,V,E> Digraph<'a,V,E> for Graph<V,E> {
    type Vertex = GraphVertexDescriptor;
    type Edge = GraphEdgeDescriptor;
    type Vertices = std::collections::hash_map::Keys<'a, Self::Vertex, V>;
    type Edges = std::collections::hash_map::Keys<'a, Self::Edge, E>;
    type Incidence = std::slice::Iter<'a, Self::Edge>;
    type Adjacency = GraphAdjacency;

    fn new() -> Self {
        return Graph {
            vertex_labels: HashMap::new(),
            edge_labels: HashMap::new(),
            out_edges: HashMap::new(),
            in_edges: HashMap::new(),
            edges: HashMap::new(),
            next_edge: GraphEdgeDescriptor(0),
            next_vertex: GraphVertexDescriptor(0)
        };
    }

    fn add_vertex(&mut self, lb: V) -> GraphVertexDescriptor {
        let n = self.next_vertex;

        self.next_vertex.0 += 1;
        self.vertex_labels.insert(n,lb);
        self.out_edges.insert(n,Vec::new());
        self.in_edges.insert(n,Vec::new());

        return n;
    }

    fn add_edge(&mut self, lb: E, from: GraphVertexDescriptor, to: GraphVertexDescriptor) -> Option<GraphEdgeDescriptor> {
        let e = self.next_edge;

        self.next_edge.0 += 1;
        self.edge_labels.insert(e,lb);
        self.edges.insert(e,(from,to));

        if self.vertex_labels.contains_key(&from) && self.vertex_labels.contains_key(&to) {
            return Some(e);
        } else {
            return None;
        }
    }

    fn remove_vertex(&mut self, v: &Self::Vertex) {
        self.vertex_labels.remove(v);

        let todel1 = match self.in_edges.get(v) {
            Some(_v) => _v.iter().map(|&x| x.clone()).collect(),
            None => Vec::new()
        };

        let todel2 = match self.out_edges.get(v) {
            Some(_v) => _v.iter().map(|&x| x.clone()).collect(),
            None => Vec::new()
        };

        for e in todel1.iter() {
            self.remove_edge(&e);
        }

        for e in todel2.iter() {
            self.remove_edge(&e);
        }

        self.out_edges.remove(v);
        self.in_edges.remove(v);
    }

    fn remove_edge(&mut self, e: &Self::Edge) {
        self.edge_labels.remove(e);

        let from = &self.source(e);
        let to = &self.target(e);

        self.edges.remove(e);

        if let Some(ie) = self.in_edges.get_mut(from) {
            loop {
                match ie.iter().position(|x| x == e) {
                    Some(i) => { ie.swap_remove(i); () },
                    None => break
                }
            }
        }

        if let Some(oe) = self.out_edges.get_mut(to) {
            loop {
                match oe.iter().position(|x| x == e) {
                    Some(i) => { oe.swap_remove(i); () },
                    None => break
                }
            }
        }
    }

    fn num_vertices(&self) -> usize {
        return self.vertex_labels.len();
    }

    fn num_edges(&self) -> usize {
        return self.edge_labels.len();
    }

    fn vertex_label(&self, n: &GraphVertexDescriptor) -> Option<&V> {
        return self.vertex_labels.get(n);
    }

    fn vertex_label_mut(&mut self, n: &GraphVertexDescriptor) -> Option<&mut V> {
        return self.vertex_labels.get_mut(n);
    }

    fn edge_label(&self, n: &GraphEdgeDescriptor) -> Option<&E> {
        return self.edge_labels.get(n);
    }

    fn edge_label_mut(&mut self, n: &GraphEdgeDescriptor) -> Option<&mut E> {
        return self.edge_labels.get_mut(n);
    }

    fn vertices(&'a self) -> Self::Vertices {
        return self.vertex_labels.keys();
    }

    fn edges(&'a self) -> Self::Edges {
        return self.edge_labels.keys();
    }

    fn out_degree(&self, v: &Self::Vertex) -> usize {
        return self.out_edges.get(v).map_or(0,|ref x| x.len());
    }

    fn in_degree(&self, v: &Self::Vertex) -> usize {
        return self.in_edges.get(v).map_or(0,|ref x| x.len());
    }

    fn degree(&self, v: &Self::Vertex) -> usize {
        return self.in_degree(v) + self.out_degree(v);
    }

    fn source(&self, e: &Self::Edge) -> Self::Vertex {
        return self.edges.get(e).unwrap().0;
    }

    fn target(&self, e: &Self::Edge) -> Self::Vertex {
        return self.edges.get(e).unwrap().1;
    }

    fn out_edges(&'a self, v: &Self::Vertex) -> Self::Incidence {
        return self.out_edges.get(v).unwrap().iter();
    }

    fn in_edges(&'a self, v: &Self::Vertex) -> Self::Incidence {
        return self.in_edges.get(v).unwrap().iter();
    }

    fn adjacent_vertices(&self, v: &Self::Vertex) -> Self::Adjacency {
        return GraphAdjacency { adj: Box::new(
            self.out_edges.get(v).unwrap().iter().map(&|&x| return self.target(&x)).chain(
                self.in_edges.get(v).unwrap().iter().map(&|&x| return self.source(&x))).collect()) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node_attribute()
    {
        let mut g = Graph::<isize,String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);

        assert!(g.vertices().any(|x| (&n1 != x) ^ (g.vertex_label(&x) == Some(&42))));
        assert!(g.vertices().any(|x| (&n2 != x) ^ (g.vertex_label(&x) == Some(&13))));
        assert!(g.vertices().any(|x| (&n3 != x) ^ (g.vertex_label(&x) == Some(&1337))));
        assert!(g.vertices().any(|ref x| g.vertex_label(x) != Some(&69)));
    }

    #[test]
    fn test_usage()
    {
        let mut g = Graph::<isize,String>::new();

        let n1 = g.add_vertex(42);
        let n2 = g.add_vertex(13);
        let n3 = g.add_vertex(1337);

        let e12 = match g.add_edge("a".to_string(),n1,n2) {
            Some(x) => x,
            None => { assert!(false); GraphEdgeDescriptor(0) }
        };
        let e23 = match g.add_edge("b".to_string(),n2,n3) {
            Some(x) => x,
            None => { assert!(false); GraphEdgeDescriptor(0) }
        };
        let e31 = match g.add_edge("c".to_string(),n3,n1) {
            Some(x) => x,
            None => { assert!(false); GraphEdgeDescriptor(0) }
        };

        assert!(n1 != n2);
        assert!(n1 != n3);
        assert!(n2 != n3);

        assert!(e12 != e23);
        assert!(e12 != e31);
        assert!(e23 != e31);

        assert!(g.vertex_label(&n1) == Some(&42));
        assert!(g.vertex_label(&n2) == Some(&13));
        assert!(g.vertex_label(&n3) == Some(&1337));

        assert!(g.edge_label(&e12) == Some(&"a".to_string()));
        assert!(g.edge_label(&e23) == Some(&"b".to_string()));
        assert!(g.edge_label(&e31) == Some(&"c".to_string()));

        assert_eq!(3,g.num_edges());
        assert_eq!(3,g.num_vertices());

        assert_eq!(g.source(&e12), n1);
        assert_eq!(g.target(&e12), n2);
        assert_eq!(g.source(&e23), n2);
        assert_eq!(g.target(&e23), n3);
        assert_eq!(g.source(&e31), n3);
        assert_eq!(g.target(&e31), n1);

        assert_eq!(g.out_degree(&n1), 1);
        assert_eq!(g.out_degree(&n2), 1);
        assert_eq!(g.out_degree(&n3), 1);

        g.remove_edge(&e12);

        g.remove_vertex(&n1);
        g.remove_vertex(&n2);
        g.remove_vertex(&n3);

        assert_eq!(g.num_vertices(), 0);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_degree()
    {
        let mut g = Graph::<Option<isize>,String>::new();

        let n1 = g.add_vertex(Some(42));
        let n2 = g.add_vertex(None);
        let n3 = g.add_vertex(Some(42));

        assert!(g.add_edge("a".to_string(),n1,n2) != None);
        let e23 = match g.add_edge("a".to_string(),n2,n3) {
            Some(x) => x,
            None => { assert!(false); GraphEdgeDescriptor(0) }
        };
        assert!(g.add_edge("a".to_string(),n3,n1) != None);

        assert_eq!(g.in_degree(&n1),1);
        assert_eq!(g.in_degree(&n2),1);
        assert_eq!(g.in_degree(&n3),1);

        assert_eq!(g.out_degree(&n1),1);
        assert_eq!(g.out_degree(&n2),1);
        assert_eq!(g.out_degree(&n3),1);

        let n4 = g.add_vertex(Some(42));
        assert!(g.add_edge("d".to_string(),n4,n1) != None);

        assert_eq!(g.in_degree(&n1),2);
        assert_eq!(g.in_degree(&n2),1);
        assert_eq!(g.in_degree(&n3),1);
        assert_eq!(g.in_degree(&n4),0);

        assert_eq!(g.out_degree(&n1),1);
        assert_eq!(g.out_degree(&n2),1);
        assert_eq!(g.out_degree(&n3),1);
        assert_eq!(g.out_degree(&n4),1);

        g.remove_edge(&e23);
        g.add_edge("d1".to_string(),n3,n2);

        let n5 = g.add_vertex(None);
        g.add_edge("d1".to_string(),n2,n5);
        g.add_edge("d2".to_string(),n5,n3);
        g.add_edge("d2".to_string(),n5,n4);

        assert_eq!(g.in_degree(&n1),2);
        assert_eq!(g.in_degree(&n2),2);
        assert_eq!(g.in_degree(&n3),1);
        assert_eq!(g.in_degree(&n4),1);
        assert_eq!(g.in_degree(&n5),1);

        assert_eq!(g.out_degree(&n1),1);
        assert_eq!(g.out_degree(&n2),1);
        assert_eq!(g.out_degree(&n3),2);
        assert_eq!(g.out_degree(&n4),1);
        assert_eq!(g.out_degree(&n5),2);

        assert_eq!(g.edges().len(),7);
    }

    /*#[test]
    fn test_out_iterator()
    {
        po::digraph<int,std::string> g;

        auto n1 = add_vertex(42,g);
        auto n2 = add_vertex(13,g);
        auto n3 = add_vertex(1337,g);
        auto n4 = add_vertex(99,g);

        auto e12 = add_edge(string("a"),n1,n2,g);
        auto e23 = add_edge(string("b"),n2,n3,g);
        auto e21 = add_edge(string("c"),n2,n1,g);
        auto e14 = add_edge(string("d"),n1,n4,g);

        auto i = out_edges(n1,g);
        ASSERT_TRUE((*i.first == e12 && *(i.first + 1) == e14) || (*i.first == e14 && *(i.first + 1) == e12));
        ASSERT_EQ(i.first + 2, i.second);

        i = out_edges(n2,g);
        ASSERT_TRUE((*i.first == e23 && *(i.first + 1) == e21) || (*i.first == e21 && *(i.first + 1) == e23));
        ASSERT_EQ(i.first + 2, i.second);

        i = out_edges(n3,g);
        ASSERT_EQ(i.first, i.second);

        i = out_edges(n4,g);
        ASSERT_EQ(i.first, i.second);
    }

    #[test]
    fn test_in_iterator()
    {
        po::digraph<int,std::string> g;

        auto n1 = add_vertex(42,g);
        auto n2 = add_vertex(13,g);
        auto n3 = add_vertex(1337,g);
        auto n4 = add_vertex(99,g);

        auto e12 = add_edge(string("a"),n1,n2,g);
        auto e23 = add_edge(string("b"),n2,n3,g);
        auto e21 = add_edge(string("c"),n2,n1,g);
        auto e14 = add_edge(string("d"),n1,n4,g);

        auto i = in_edges(n1,g);
        ASSERT_TRUE(*i.first == e21);
        ASSERT_EQ(i.first + 1, i.second);

        i = in_edges(n2,g);
        ASSERT_TRUE(*i.first == e12);
        ASSERT_EQ(i.first + 1, i.second);

        i = in_edges(n3,g);
        ASSERT_TRUE(*i.first == e23);
        ASSERT_EQ(i.first + 1, i.second);

        i = in_edges(n4,g);
        ASSERT_TRUE(*i.first == e14);
        ASSERT_EQ(i.first + 1, i.second);
    }

    #[test]
    fn test_adj_iterator()
    {
        po::digraph<int,std::string> g;

        auto n1 = add_vertex(42,g);
        auto n2 = add_vertex(13,g);
        auto n3 = add_vertex(1337,g);
        auto n4 = add_vertex(99,g);

        add_edge(string("a"),n1,n2,g);
        add_edge(string("b"),n2,n3,g);
        add_edge(string("c"),n2,n1,g);
        add_edge(string("d"),n1,n4,g);

        auto i = adjacent_vertices(n1,g);
        ASSERT_TRUE((*i.first == n2 && *(i.first + 1) == n4) || (*i.first == n4 && *(i.first + 1) == n2));
        ASSERT_EQ(std::distance(i.first ,i.second), 2);

        i = adjacent_vertices(n2,g);
        ASSERT_TRUE((*i.first == n1 && *(i.first + 1) == n3) || (*i.first == n3 && *(i.first + 1) == n1));
        ASSERT_EQ(std::distance(i.first ,i.second), 2);

        i = adjacent_vertices(n3,g);
        ASSERT_TRUE(*i.first == n2);
        ASSERT_EQ(std::distance(i.first ,i.second), 1);

        i = adjacent_vertices(n4,g);
        ASSERT_TRUE(*i.first == n1);
        ASSERT_EQ(std::distance(i.first ,i.second), 1);
    }

    #[test]
    fn test_iterators()
    {
        po::digraph<int,std::string> g;

        auto n1 = add_vertex(42,g);
        auto n2 = add_vertex(13,g);
        auto n3 = add_vertex(1337,g);
        auto n4 = add_vertex(99,g);

        add_edge(string("a"),n1,n2,g);
        add_edge(string("b"),n2,n3,g);
        add_edge(string("c"),n2,n1,g);
        add_edge(string("d"),n1,n4,g);

        auto i = vertices(g);
        std::set<decltype(g)::vertex_descriptor> ns;
        std::for_each(i.first,i.second,[&](const decltype(g)::vertex_descriptor &n) { ASSERT_TRUE(ns.insert(n).second); });

        auto j = edges(g);
        std::set<decltype(g)::edge_descriptor> es;
        std::for_each(j.first,j.second,[&](const decltype(g)::edge_descriptor &n) { ASSERT_TRUE(es.insert(n).second); });

        ASSERT_EQ(ns.size(), 4u);
        ASSERT_EQ(es.size(), 4u);
    }

    #[test]
    fn test_error()
    {
        po::digraph<int,std::string> g1,g2;

        auto n1 = add_vertex(42,g1);
        auto n2 = add_vertex(13,g1);
        add_vertex(13,g1);

        add_edge(string("a"),n1,n2,g1);
        add_edge(string("b"),n1,n2,g1);

        ASSERT_EQ(num_edges(g1), 2u);
        ASSERT_EQ(num_vertices(g1), 3u);
    }

    #[test]
    fn test_remove_edge_from_node_with_multiple_out_edges()
    {
        po::digraph<int,std::string> g;

        auto n1 = add_vertex(42,g);
        auto n2 = add_vertex(13,g);
        auto n3 = add_vertex(12,g);

        auto e12 = add_edge(string("a"),n1,n2,g);
        add_edge(string("b"),n1,n3,g);

        ASSERT_EQ(num_edges(g), 2u);
        ASSERT_EQ(num_vertices(g), 3u);
        ASSERT_EQ(out_degree(n1,g), 2u);

        remove_edge(e12,g);

        ASSERT_EQ(out_degree(n1,g), 1u);
    }*/
}
