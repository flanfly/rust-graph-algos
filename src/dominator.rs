use std::iter::FromIterator;
use std::collections::{
    HashMap,
};
use traits::{
    Graph,
    VertexListGraph,
    BidirectionalGraph,
};
use search::{
    TreeIterator,
    TraversalOrder
};
use bit_set::BitSet;

pub fn dominators<'a, V, E, G: 'a + Graph<'a,V,E> + BidirectionalGraph<'a,V,E> + VertexListGraph<'a,V,E>>(start: G::Vertex, graph: &'a G) -> HashMap<G::Vertex,Vec<G::Vertex>> {
    let vertex_idx = HashMap::<G::Vertex,usize>::from_iter(graph.vertices().enumerate().map(|(a,b)| (b,a)));
    let vertex_ridx = HashMap::<usize,G::Vertex>::from_iter(graph.vertices().enumerate());
    let mut cur_dom = vec![BitSet::new(); vertex_idx.len()];
    let mut fixpoint = false;

    for vx in (0..vertex_idx.len()) {
        cur_dom[vx].insert(vx);
    }

    while !fixpoint {
        let mut next_dom = vec![BitSet::new(); vertex_idx.len()];
        println!("cur {:?}",cur_dom);

        for vx in graph.vertices() {
            let mut my_dom: Option<BitSet> = None;

            for e in graph.in_edges(vx) {
                let prev = &cur_dom[vertex_idx[&graph.source(e)]];

                if let Some(ref mut s) = my_dom {
                    s.intersect_with(&prev);
                } else {
                    my_dom = Some(prev.clone());
                }
            }

            let mut final_dom = my_dom.unwrap_or(BitSet::new());
            let vx_idx = vertex_idx[&vx];

            final_dom.union_with(&cur_dom[vx_idx]);
            next_dom[vx_idx] = final_dom;
        }

        println!("next {:?}",next_dom);
        fixpoint = next_dom == cur_dom;
        cur_dom = next_dom;
    }

    let mut ret = HashMap::<G::Vertex,Vec<G::Vertex>>::new();
    for (vx,idx) in vertex_idx.iter() {
        let mut res = cur_dom[*idx].iter().map(|a| vertex_ridx[&a]).collect::<Vec<G::Vertex>>();

        res.sort();
        ret.insert(*vx,res);
    }

    ret
}
/*
pub fn domiance_frontier(doms,G) -> HashMap<G::Vertex,Vec<G::Vertex>> {
}

pub fn itermediate_domiator*/

#[cfg(test)]
mod tests {
    use super::*;
    use adjacency_list::{
        AdjacencyList,
    };
    use traits::{
        MutableGraph,
    };

    #[test]
    fn dom() {
        let mut g = AdjacencyList::<usize,()>::new();
        let v1 = g.add_vertex(1);
        let v2 = g.add_vertex(2);
        let v3 = g.add_vertex(3);
        let v4 = g.add_vertex(4);
        let v5 = g.add_vertex(5);
        let v6 = g.add_vertex(6);

        g.add_edge((),v1,v2);
        g.add_edge((),v2,v3);
        g.add_edge((),v2,v4);
        g.add_edge((),v2,v6);
        g.add_edge((),v3,v5);
        g.add_edge((),v4,v5);
        g.add_edge((),v5,v2);

        let dom = dominators(v1,&g);

        println!("{:?}",dom);

        assert_eq!(dom.len(), 6);
        assert_eq!(dom[&v1], vec![v1]);
        assert_eq!(dom[&v2], vec![v1,v2]);
        assert_eq!(dom[&v3], vec![v1,v2,v3]);
        assert_eq!(dom[&v4], vec![v1,v2,v4]);
        assert_eq!(dom[&v5], vec![v1,v2,v5]);
        assert_eq!(dom[&v6], vec![v1,v2,v6]);
    }
}

