// find_province_num_bfs

use std::collections::HashMap;
use std::hash::Hash;

use crate::data::Queue;

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

    fn add_neighbor(&mut self, nbr: T) {
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
    edges: HashMap<T, Vec<T>>,
}

impl<T: Eq + PartialEq + Clone + Hash> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
            edges: HashMap::<T, Vec<T>>::new(),
        }
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn add_edge(&mut self, src: &T, des: &T) {
        if !self.vertices.contains_key(src) {
            let _sv = self.add_vertex(src);
        }

        if !self.vertices.contains_key(des) {
            let _dv = self.add_vertex(des);
        }

        self.edgenums += 1;
        self.vertices.get_mut(src)
                        .unwrap()
                        .add_neighbor(des.clone());

        if !self.edges.contains_key(src) {
            let _eg = self.edges.insert(src.clone(), Vec::new());
        }

        self.edges.get_mut(src)
                    .unwrap()
                    .push(des.clone());
    }
}

// 构 建 城 市 连 接 关 系 图
fn build_city_graph<T>(connected: Vec<Vec<T>>) -> Graph<T>
where T: Eq + PartialEq + Clone + Hash {
    // 有 关 联 关 系 的 城 市 节 点 间 设 置 边
    let mut city_graph = Graph::new();
    for v in connected.iter() {
        let src = v.first().unwrap();
        let des = v.last().unwrap();
        city_graph.add_edge(src, des);
    }

    city_graph
}

fn find_province_num_bfs<T>(connected: Vec<Vec<T>>) -> u32
where T: Eq + PartialEq + Clone + Hash {
    let mut cg = build_city_graph(connected);

    // 获 取 各 个 主 节 点 城 市 key
    let mut cities = Vec::new();
    for key in cg.edges.keys() { cities.push(key.clone()); }
    // 逐 个 处 理 省 强 连 通 分 量
    let mut province_num = 0;
    let mut q = Queue::new(cities.len());
    for ct in &cities {
        let city = cg.vertices.get(ct).unwrap().clone();
        if Color::White == city.color {
            // 改 变 当 前 节 点 颜 色 并 入 队
            cg.vertices.get_mut(ct)
                        .unwrap()
                        .color = Color::Gray;
                        q.enqueue(city);
            // 处 理 一 个 省 强 连 通 分 量
            while !q.is_empty() {
                // 获 取 某 节 点 及 其 邻 点
                let q_city = q.dequeue().unwrap();
                let nbrs = q_city.get_neighbors();
                // 逐 个 处 理 邻 点
                for nbr in nbrs {
                    let nbrc = cg.vertices.get(nbr)
                    .unwrap()
                    .clone();
                    if Color::White == nbrc.color {
                        // 当 前 节 点 邻 点 未 被 探 索 过 ， 入队
                        cg.vertices.get_mut(nbr)
                                    .unwrap()
                                    .color = Color::Gray;
                                    q.enqueue(nbrc);
                    }
                }
            }
            // 处 理 完 一 个 省 强 连 通 分 量
            province_num += 1;
        }
    }

    province_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_province() {
        // 构 建 城 市 依 赖 关 系
        let mut connected = Vec::<Vec<&str>>::new();
        connected.push(vec!["成都", "自贡"]);
        connected.push(vec!["成都", "绵阳"]);
        connected.push(vec!["成都", "德阳"]);
        connected.push(vec!["成都", "泸州"]);
        connected.push(vec!["成都", "内江"]);
        connected.push(vec!["成都", "乐山"]);
        connected.push(vec!["成都", "宜宾"]);
        connected.push(vec!["自贡", "成都"]);
        connected.push(vec!["广州", "深圳"]);
        connected.push(vec!["广州", "东莞"]);
        connected.push(vec!["广州", "珠海"]);
        connected.push(vec!["广州", "中山"]);
        connected.push(vec!["广州", "汕头"]);
        connected.push(vec!["广州", "佛山"]);
        connected.push(vec!["广州", "湛江"]);
        connected.push(vec!["深圳", "广州"]);
        connected.push(vec!["武汉", "荆州"]);
        connected.push(vec!["武汉", "宜昌"]);
        connected.push(vec!["武汉", "襄阳"]);
        connected.push(vec!["武汉", "荆门"]);
        connected.push(vec!["武汉", "孝感"]);
        connected.push(vec!["武汉", "黄冈"]);
        connected.push(vec!["荆州", "武汉"]);
        // 找 到 所 有 的 省 强 连 通 分 量 ， 有三个省 ： 四川 、 广东 、 湖北
        let province_num = find_province_num_bfs(connected);
        println!("province number: {province_num}");
        // province number: 3
    }

    #[test]
    fn test_shuixianhua() {
        // let num = 467;
        // let g: i32 = num % 10;
        // let s: i32 = num / 10 % 10;
        // let b: i32 = num / 100;

        // let out = g.pow(3) + s.pow(3) + b.pow(3);
        // println!("{}", out);
        for num in 100..1000 {
            let g: i32 = num % 10;
            let s: i32 = num / 10 % 10;
            let b: i32 = num / 100;

            let out = g.pow(3) + s.pow(3) + b.pow(3);

            if out == num {
                println!("find {}!", num);
            }

        }
    }

    #[test]
    fn test_jitutonglong() {
        for i in 1..100 {
            // 10
            for j in 1..100 {
                // 5
                for k in 1..100 {
                    // 2
                    if i*10 + j*5 + k*2 == 100 {
                        println!("i:{}, j:{}, k:{}", i, j, k);
                    }
                }
            }
        }
    }
}

