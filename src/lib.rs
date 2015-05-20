use std::collections::HashMap;
use std::slice::Iter;

#[derive(PartialEq,Eq,Hash,Copy,Clone)]
pub struct GraphVertexDescriptor(pub usize);
#[derive(PartialEq,Eq,Hash,Copy,Clone)]
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

    /*fn test_usage()
    {
        po::digraph<int,std::string> g;

        auto n1 = add_vertex(42,g);
        auto n2 = add_vertex(13,g);
        auto n3 = add_vertex(1337,g);

        auto e12 = add_edge(string("a"),n1,n2,g);
        auto e23 = add_edge(string("b"),n2,n3,g);
        auto e31 = add_edge(string("c"),n3,n1,g);

        ASSERT_NE(n1, n2);
        ASSERT_NE(n1, n3);
        ASSERT_NE(n2, n3);

        ASSERT_NE(e12, e23);
        ASSERT_NE(e12, e31);
        ASSERT_NE(e23, e31);

        ASSERT_EQ(get_vertex(n1,g), 42);
        ASSERT_EQ(get_vertex(n2,g), 13);
        ASSERT_EQ(get_vertex(n3,g), 1337);

        ASSERT_EQ(get_edge(e12,g), string("a"));
        ASSERT_EQ(get_edge(e23,g), string("b"));
        ASSERT_EQ(get_edge(e31,g), string("c"));

        ASSERT_EQ(num_edges(g), 3u);
        ASSERT_EQ(num_vertices(g), 3u);

        ASSERT_EQ(source(e12,g), n1);
        ASSERT_EQ(target(e12,g), n2);
        ASSERT_EQ(source(e23,g), n2);
        ASSERT_EQ(target(e23,g), n3);
        ASSERT_EQ(source(e31,g), n3);
        ASSERT_EQ(target(e31,g), n1);

        ASSERT_EQ(out_degree(n1,g), 1u);
        ASSERT_EQ(out_degree(n2,g), 1u);
        ASSERT_EQ(out_degree(n3,g), 1u);

        remove_edge(e12,g);

        remove_vertex(n1,g);
        remove_vertex(n2,g);
        remove_vertex(n3,g);

        ASSERT_EQ(num_vertices(g), 0u);
        ASSERT_EQ(num_edges(g), 0u);
    }

    fn test_degree()
    {
        po::digraph<boost::optional<int>,std::string> g;

        auto n1 = add_vertex(boost::make_optional(42),g);
        auto n2 = add_vertex(boost::optional<int>(boost::none),g);
        auto n3 = add_vertex(boost::make_optional(42),g);

        add_edge(string("a"),n1,n2,g);
        auto e23 = add_edge(string("a"),n2,n3,g);
        add_edge(string("a"),n3,n1,g);

        ASSERT_EQ(in_degree(n1,g),1u);
        ASSERT_EQ(in_degree(n2,g),1u);
        ASSERT_EQ(in_degree(n3,g),1u);

        ASSERT_EQ(out_degree(n1,g),1u);
        ASSERT_EQ(out_degree(n2,g),1u);
        ASSERT_EQ(out_degree(n3,g),1u);

        auto n4 = add_vertex(boost::make_optional(42),g);
        add_edge(string("d"),n4,n1,g);

        ASSERT_EQ(in_degree(n1,g),2u);
        ASSERT_EQ(in_degree(n2,g),1u);
        ASSERT_EQ(in_degree(n3,g),1u);
        ASSERT_EQ(in_degree(n4,g),0u);

        ASSERT_EQ(out_degree(n1,g),1u);
        ASSERT_EQ(out_degree(n2,g),1u);
        ASSERT_EQ(out_degree(n3,g),1u);
        ASSERT_EQ(out_degree(n4,g),1u);

        remove_edge(e23,g);
        add_edge(string("d1"),n3,n2,g);

        auto n5 = add_vertex(boost::optional<int>(boost::none),g);
        add_edge(string("d1"),n2,n5,g);
        add_edge(string("d2"),n5,n3,g);
        add_edge(string("d2"),n5,n4,g);

        ASSERT_EQ(in_degree(n1,g),2u);
        ASSERT_EQ(in_degree(n2,g),2u);
        ASSERT_EQ(in_degree(n3,g),1u);
        ASSERT_EQ(in_degree(n4,g),1u);
        ASSERT_EQ(in_degree(n5,g),1u);

        ASSERT_EQ(out_degree(n1,g),1u);
        ASSERT_EQ(out_degree(n2,g),1u);
        ASSERT_EQ(out_degree(n3,g),2u);
        ASSERT_EQ(out_degree(n4,g),1u);
        ASSERT_EQ(out_degree(n5,g),2u);

        auto p = edges(g);
        ASSERT_EQ(std::distance(p.first,p.second),7);
    }

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
