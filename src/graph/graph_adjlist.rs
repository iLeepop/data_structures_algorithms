/// graph_adjlist


use std::hash::Hash;
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Vertex<T> {
    key: T,
    connects: Vec<(T, i32)>,
}

impl<T: Clone + PartialEq> Vertex<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            connects: Vec::new()
        }
    }

    pub fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.connects.iter() {
            if nbr == key { return true; }
        }

        false
    }

    fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.connects.push((nbr, wt));
    }

    pub fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr);
        }

        connects
    }

    pub fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }

        &0
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>,
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.vertnums
    }

    pub fn vertex_num(&self) -> u32 {
        self.vertnums
    }

    pub fn edge_num(&self) -> u32 {
        self.edgenums
    }

    pub fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key {
                return true;
            }
        }

        false
    }

    pub fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    pub fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        if let Some(vertex) = self.vertices.get(key) {
            Some(&vertex)
        } else {
            None
        }
    }

    pub fn vertex_keys(&self) -> Vec<T> {
        let mut keys = Vec::new();
        for key in self.vertices.keys() {
            keys.push(key.clone());
        }

        keys
    }

    pub fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertices.remove(key);
        self.vertnums -= 1;

        self.edgenums -= old_vertex.clone()
                                    .unwrap()
                                    .get_connects()
                                    .len() as u32;
        for vertex in self.vertex_keys() {
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.connects.retain(|(k, _)| k != key);
                    self.edgenums -= 1;
                }
            }
        }

        old_vertex
    }

    pub fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        if !self.contains(from) {
            let _fvert = self.add_vertex(from);
        }

        if !self.contains(to) {
            let _tvert = self.add_vertex(to);
        }

        self.edgenums += 1;
        self.vertices.get_mut(from)
                        .unwrap()
                        .add_neighbor(to.clone(), wt);
    }

    pub fn adjacent(&self, from: &T, to: &T) -> bool {
        self.vertices.get(from).unwrap().adjacent_key(to)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_graph() {
        let mut g = Graph::new();
        for i in 0..6 { g.add_vertex(&i); }
        println!("graph empty: {}", g.is_empty());
        let vertices = g.vertex_keys();
        for vertex in vertices { println!("Vertex: {:#?}", vertex); }
        g.add_edge(&0,&1,5); g.add_edge(&0,&5,2);
        g.add_edge(&1,&2,4); g.add_edge(&2,&3,9);
        g.add_edge(&3,&4,7); g.add_edge(&3,&5,3);
        g.add_edge(&4,&0,1); g.add_edge(&4,&4,8);
        println!("vert nums: {}", g.vertex_num());
        println!("edge nums: {}", g.edge_num());
        println!("contains 0: {}", g.contains(&0));
        let vertex = g.get_vertex(&0).unwrap();
        println!("key: {}, to nbr 1 weight: {}",
        vertex.key, vertex.get_nbr_weight(&1));
        let keys = vertex.get_connects();
        for nbr in keys { println!("nighbor: {nbr}"); }
        for (nbr, wt) in vertex.connects.iter() {
            println!("0 nighbor: {nbr}, weight: {wt}");
        }
        let res = g.adjacent(&0, &1);
        println!("0 adjacent to 1: {res}");
        let res = g.adjacent(&3, &2);
        println!("3 adjacent to 2: {res}");

        let rm = g.remove_vertex(&0).unwrap();
        println!("remove vertex: {}", rm.key);
        println!("left vert nums: {}", g.vertex_num());
        println!("left edge nums: {}", g.edge_num());
        println!("contains 0: {}", g.contains(&0));
    }
}


