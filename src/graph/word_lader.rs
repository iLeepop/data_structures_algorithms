/// word lader

use std::collections::HashMap;
use std::hash::Hash;

use crate::data::Queue;




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
fn word_ladder(g: &mut Graph<String>, start: Vertex<String>, end: Vertex<String>, len: usize) -> u32 {
    if !g.vertices.contains_key(&start.key) { return 0; }
    if !g.vertices.contains_key(&end.key) { return 0; }

    let mut vertex_queue = Queue::new(len);
    let _r = vertex_queue.enqueue(start);

    while vertex_queue.size() > 0 {
        let curr = vertex_queue.dequeue().unwrap();
        for nbr in curr.get_neighbors() {
            let mut nbv = g.vertices.get(nbr).unwrap().clone();
            if end.key != nbv.key {
                if Color::White == nbv.color {
                    nbv.color = Color::Gray;
                    nbv.distance = curr.distance + 1;

                    g.vertices.get_mut(nbr)
                                .unwrap()
                                .color = Color::Gray;
                    g.vertices.get_mut(nbr)
                                .unwrap()
                                .distance = curr.distance + 1;
                    
                    let _r = vertex_queue.enqueue(nbv);
                }
            } else {
                return curr.distance + 1;
            }
        }
    }

    0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_word_ladder() {
        let words = vec![
            "FOOL", "COOL", "POOL", "FOUL", "FOIL", "FAIL", "FALL",
            "POLL", "PALL", "POLE", "PALE", "SALE", "PAGE", "SAGE",
        ];
        let len = words.len();
        let mut g = build_word_graph(words);

        // 首 节 点 加 入 队 列 表 明 正 被 探 索 ， 所 以 颜 色 变 为 灰 色
        g.vertices.get_mut("FOOL").unwrap().color = Color::Gray;

        // 取出 首 尾点
        let start = g.vertices.get("FOOL").unwrap().clone();
        let end = g.vertices.get("SAGE").unwrap().clone();
        // 计 算 最 小 转 换 次 数 ， 也 就 是 距离
        let distance = word_ladder(&mut g, start, end, len);
        println!("the shortest distance: {distance}");
        // the shortest distance: 6
    }
}

