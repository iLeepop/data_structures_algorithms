/// word lader

use std::collections::HashMap;
use std::hash::Hash;


#[derive(Clone, Debug, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}


#[derive(Debug, Clone)]
struct Vertex<T> {
    color: Color,
    distance: u32,
    key: T,
    neighbors: Vec<(T, u32)>,
}


impl<T: Clone + PartialEq> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            color: Color::White, 
            distance: 0,
            key,
            neighbors: Vec::new(),
        }
    }

    fn add_neighbor(&mut self, nbr: T, wt: u32) {
        self.neighbors.push((nbr, wt));
    }

    fn get_neighbors(&self) -> Vec<&T> {
        let mut neighbors = Vec::new();
        for (nbr, _wt) in self.neighbors.iter() {
            neighbors.push(nbr);
        }
        neighbors
    }
}

#[derive(Debug, Clone)]
struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>,
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn add_edge(&mut self, from: &T, to: &T, wt: u32) {
        if !self.contains(from) {
            let _fvert = self.add_vertex(from);
        }
        if !self.contains(to) {
            let _tvert = self.add_vertex(to);
        }

        self.edgenums += 1;
        self.vertices
                .get_mut(from)
                .unwrap()
                .add_neighbor(to.clone(), wt);
    }
}


// 构建图
fn build_word_graph(words: Vec<&str>) -> Graph<String> {
    let mut hmap: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        for i in 0..word.len() {
            let pattn = word[..i].to_string() + "_" + &word[i + 1..];
            if hmap.contains_key(&pattn) {
                hmap.get_mut(&pattn)
                    .unwrap()
                    .push(word.to_string());
            } else {
                hmap.insert(pattn, vec![word.to_string()]);
            }
        }
    }
    let mut word_graph = Graph::new();
        for word in hmap.keys() {
            for w1 in &hmap[word] {
                for w2 in &hmap[word] {
                    if w1 != w2 {
                        word_graph.add_edge(w1, w2, 1);
                    }
                }
            }
        }
    word_graph
}


// 字梯图 - 广度优先搜索
fn word_ladder() {
    
}

