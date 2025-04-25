// kbight_tour 骑士之旅

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;


const BDSIZE: u32 = 8;


#[derive(Debug, Clone, PartialEq)]
enum Color {
    White,
    Gray,
}

#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    color: Color,
    neighbors: Vec<T>,
}

impl<T: PartialEq + Clone> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            color: Color::White,
            neighbors: Vec::new(),
        }
    }

    fn add_neighbors(&mut self, nbr: T) {
        self.neighbors.push(nbr);
    }

    fn get_neighbors(&self) -> Vec<&T> {
        let mut neighbors = Vec::new();
        for nbr in self.neighbors.iter() {
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

impl<T: Eq + PartialEq + Clone + Hash> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T,Vertex<T>>::new(),
        }
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn add_edge(&mut self, src: &T, des: &T) {
        if !self.vertices.contains_key(src) {
            let _fv = self.add_vertex(src);
        }
        if !self.vertices.contains_key(des) {
            let _tv = self.add_vertex(des);
        }

        self.edgenums += 1;
        self.vertices.get_mut(src)
                        .unwrap()
                        .add_neighbors(des.clone());
    }
}

fn legal_moves(x: u32, y: u32, bdsize: u32) -> Vec<(u32, u32)> {
    let move_offsets = [
        (-1, 2), (1, 2),
        (-2, 1), (2, 1),
        (-2, -1), (2, -1),
        (-1, -2), (1, -2),
    ];

    let legal_pos = |a: i32, b: i32| { a >= 0 && a < b };

    let mut legal_positions = Vec::new();
    for (x_offset, y_offset) in move_offsets.iter() {
        let new_x = x as i32 + x_offset;
        let new_y = y as i32 + y_offset;

        if legal_pos(new_x, bdsize as i32)
            && legal_pos(new_y, bdsize as i32) {
                legal_positions.push((new_x as u32, new_y as u32));
            }
    }

    legal_positions
}

fn build_knight_graph(bdsize: u32) -> Graph<u32> {
    let calc_point = |row: u32, col: u32, size: u32| {
        (row % size) * size + col
    };

    let mut knight_graph = Graph::new();
    for row in 0..bdsize {
        for col in 0..bdsize {
            let dests = legal_moves(row, col, bdsize);
            for des in dests {
                let src_p = calc_point(row, col, bdsize);
                let des_p = calc_point(des.0, des.1, bdsize);
                knight_graph.add_edge(&src_p, &des_p);
            }
        }
    }
    
    knight_graph
}

fn knight_tour<T>(kg: &mut Graph<T>, curr: Vertex<T>, path: &mut Vec<String>, depth: u32) -> bool
where T: Eq + PartialEq + Clone + Hash + Display
{
    path.push(curr.key.to_string());

    let mut done = false;
    if depth < BDSIZE * BDSIZE - 1 {
        let mut i = 0;
        let nbrs = curr.get_neighbors();

        while i < nbrs.len() && !done {
            let nbr = kg.vertices.get(nbrs[i]).unwrap().clone();

            if Color::White == nbr.color {
                kg.vertices.get_mut(nbrs[i])
                            .unwrap()
                            .color = Color::Gray;
                
                done = knight_tour(kg, nbr, path, depth + 1);
                if !done {
                    let _rm = path.pop();
                    kg.vertices.get_mut(nbrs[i])
                                .unwrap()
                                .color = Color::White;
                }
            }

            i += 1;
        }
    } else {
        done = true;
    }

    done
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tset_knight_tour() {
        let mut kg: Graph<u32> = build_knight_graph(BDSIZE);

        // 选 择 起 始 点 并 更 新 图 中 点 颜 色
        let point = 0;
        kg.vertices.get_mut(&point).unwrap().color = Color::Gray;
        let start = kg.vertices.get(&point).unwrap().clone();
        
        // 开 始 骑 士 之 旅 ，path 保 存 所 有 访 问 过 的 点
        let mut path = Vec::new();
        let successed = knight_tour(&mut kg, start, &mut path, 0);

        // 将 结 果 格 式 化 输 出
        if successed {
        for row in 0..BDSIZE {
        let row_s = ((row % BDSIZE) * BDSIZE) as usize;
        let row_e = row_s + BDSIZE as usize;
        let row_str = path[row_s..row_e].join("\t");
        println!("{row_str}");
        }
        }
    }
}
