use std::iter::FromIterator;
use std::collections::{
    HashMap,
};
use search::{
    TreeIterator,
    TraversalOrder,
};
use traits::{
    Graph,
    VertexListGraph,
    BidirectionalGraph,
};
use bit_set::BitSet;

pub fn dominators<'a, V, E, G: 'a + Graph<'a,V,E> + BidirectionalGraph<'a,V,E> + VertexListGraph<'a,V,E>>(start: G::Vertex, graph: &'a G) -> HashMap<G::Vertex,Vec<G::Vertex>> {
    let vertex_idx = HashMap::<G::Vertex,usize>::from_iter(graph.vertices().enumerate().map(|(a,b)| (b,a)));
    let vertex_ridx = HashMap::<usize,G::Vertex>::from_iter(graph.vertices().enumerate());
    let mut fixpoint = false;
    let mut all_set = BitSet::new();

    for vx in 0..vertex_idx.len() {
        all_set.insert(vx);
    }

    let mut cur_dom = vec![all_set; vertex_idx.len()];

    while !fixpoint {
        let mut next_dom = vec![BitSet::new(); vertex_idx.len()];

        for vx in graph.vertices() {
            let mut my_dom: Option<BitSet> = None;

            if vx != start {
                for e in graph.in_edges(vx) {
                    let prev = &cur_dom[vertex_idx[&graph.source(e)]];

                    if let Some(ref mut s) = my_dom {
                        s.intersect_with(&prev);
                    } else {
                        my_dom = Some(prev.clone());
                    }
                }
            }

            let mut final_dom = my_dom.unwrap_or(BitSet::new());
            let vx_idx = vertex_idx[&vx];

            final_dom.insert(vx_idx);
            next_dom[vx_idx] = final_dom;
        }

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

/// Cooper, Harvey, Kennedy: "A Simple, Fast Dominance Algorithm"
pub fn dominance_frontiers<'a, V, E, G: 'a + Graph<'a,V,E> + BidirectionalGraph<'a,V,E> + VertexListGraph<'a,V,E>>(idom: &HashMap<G::Vertex,G::Vertex>, graph: &'a G) -> HashMap<G::Vertex,Vec<G::Vertex>> {
    let mut ret = HashMap::<G::Vertex,Vec<G::Vertex>>::from_iter(graph.vertices().map(|v| (v,vec![])));

    for b in graph.vertices() {
        if graph.in_degree(b) >= 2 {
            for e in graph.in_edges(b) {
                let p = graph.source(e);
                let mut runner = p;

                while runner != idom[&b] {
                    ret.entry(runner).or_insert(vec![]).push(b);
                    runner = idom[&runner];
                }
            }
        }
    }

    for (_,v) in ret.iter_mut() {
        v.sort();
        v.dedup();
    }

    ret
}

/// Cooper, Harvey, Kennedy: "A Simple, Fast Dominance Algorithm"
pub fn immediate_dominator<'a, V, E, G: 'a + Graph<'a,V,E> + BidirectionalGraph<'a,V,E> + VertexListGraph<'a,V,E>>(start: G::Vertex, graph: &'a G) -> HashMap<G::Vertex,G::Vertex> {
    let mut rev_postorder = TreeIterator::new(start,TraversalOrder::Postorder,graph).collect::<Vec<_>>();
    rev_postorder.reverse();

    let rpo_idx = HashMap::<G::Vertex,usize>::from_iter(rev_postorder.iter().enumerate().map(|(a,b)| (b.clone(),a)));
    fn intersect<'a, V, E, G: 'a + Graph<'a,V,E> + BidirectionalGraph<'a,V,E> + VertexListGraph<'a,V,E>>(p: G::Vertex,q: G::Vertex,rpo_idx: &HashMap<G::Vertex,usize>, rev_postorder: &Vec<G::Vertex>, ret: &HashMap<G::Vertex,G::Vertex> ) -> G::Vertex {
        let mut f1 = rpo_idx[&p];
        let mut f2 = rpo_idx[&q];

        while f1 != f2 {
            while f1 > f2 {
                f1 = rpo_idx[&ret[&rev_postorder[f1]]];
            }
            while f1 < f2 {
                f2 = rpo_idx[&ret[&rev_postorder[f2]]];
            }
        }

        rev_postorder[f1]
    };

    let mut ret = HashMap::<G::Vertex,G::Vertex>::new();
    let mut fixpoint = false;

    ret.insert(start,start);

    while !fixpoint {
        fixpoint = true;

        for b in rev_postorder.iter().filter(|&&v| v != start) {
            let mut new_idom = None;

            for e in graph.in_edges(*b) {
                let p = graph.source(e);

                if p != *b {
                    if let Some(ref mut d) = new_idom {
                        if ret.contains_key(&p) {
                            *d = intersect::<V,E,G>(p,*d,&rpo_idx,&rev_postorder,&ret);
                        }
                    } else {
                        new_idom = Some(p);
                    }
                }
            }

            assert!(new_idom.is_some());

            if ret.get(b) != new_idom.as_ref() {
                ret.insert(*b,new_idom.unwrap());
                fixpoint = false;
            }
        }
    }

    ret
}

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

    #[test]
    fn idom() {
        let mut g = AdjacencyList::<usize,()>::new();
        let v1 = g.add_vertex(1);
        let v2 = g.add_vertex(2);
        let v3 = g.add_vertex(3);
        let v4 = g.add_vertex(4);
        let v5 = g.add_vertex(5);
        let v6 = g.add_vertex(6);

        g.add_edge((),v6,v5);
        g.add_edge((),v6,v4);
        g.add_edge((),v5,v1);
        g.add_edge((),v4,v2);
        g.add_edge((),v4,v3);
        g.add_edge((),v3,v2);
        g.add_edge((),v2,v3);
        g.add_edge((),v1,v2);
        g.add_edge((),v2,v1);

        let doms = immediate_dominator(v6,&g);

        assert_eq!(doms.len(), 6);
        assert_eq!(doms[&v1], v6);
        assert_eq!(doms[&v2], v6);
        assert_eq!(doms[&v3], v6);
        assert_eq!(doms[&v4], v6);
        assert_eq!(doms[&v5], v6);
        assert_eq!(doms[&v6], v6);
    }

    #[test]
    fn frontiers() {
        let mut g = AdjacencyList::<usize,()>::new();
        let v0 = g.add_vertex(0);
        let v1 = g.add_vertex(1);
        let v2 = g.add_vertex(2);
        let v3 = g.add_vertex(3);
        let v4 = g.add_vertex(4);
        let v5 = g.add_vertex(5);
        let v6 = g.add_vertex(6);
        let v7 = g.add_vertex(7);
        let v8 = g.add_vertex(8);

        g.add_edge((),v0,v1);
        g.add_edge((),v1,v2);
        g.add_edge((),v1,v5);
        g.add_edge((),v5,v6);
        g.add_edge((),v5,v8);
        g.add_edge((),v6,v7);
        g.add_edge((),v8,v7);
        g.add_edge((),v2,v3);
        g.add_edge((),v7,v3);
        g.add_edge((),v3,v4);
        g.add_edge((),v3,v1);

        let idom = immediate_dominator(v0,&g);
        let fron = dominance_frontiers(&idom,&g);

        assert_eq!(fron.len(), 9);
        assert_eq!(fron[&v0], vec![]);
        assert_eq!(fron[&v1], vec![v1]);
        assert_eq!(fron[&v2], vec![v3]);
        assert_eq!(fron[&v3], vec![v1]);
        assert_eq!(fron[&v4], vec![]);
        assert_eq!(fron[&v5], vec![v3]);
        assert_eq!(fron[&v6], vec![v7]);
        assert_eq!(fron[&v7], vec![v3]);
        assert_eq!(fron[&v8], vec![v7]);
    }
}
