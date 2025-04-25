// topological_sort 拓扑

use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
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

fn build_graph<T>(pre_requisites: Vec<Vec<T>>) -> Graph<T>
where T: Eq + PartialEq + Clone + Hash {
    let mut graph = Graph::new();
    for v in pre_requisites.iter() {
        let prev = v.first().unwrap();
        let last = v.last().unwrap();
        graph.add_edge(prev, last);
    }
    graph
}

fn scheduling<T>(
    cg: &mut Graph<T>,
    course: Vertex<T>,
    schedule: &mut Vec<String>,
    mut has_circle: bool
)
where T: Eq + PartialEq + Clone + Hash + Display {
    let edges = cg.edges.clone();

    let dependencies = edges.get(&course.key);
    if !dependencies.is_none() {
        for dep in dependencies.unwrap().iter() {
            let course = cg.vertices.get(dep).unwrap().clone();
            if Color::White == course.color {
                cg.vertices.get_mut(dep)
                            .unwrap()
                            .color = Color::Gray;
                scheduling(cg, course, schedule, has_circle);

                if has_circle { return; }
            } else if Color::Gray == course.color {
                has_circle = true;
                return;
            }
        }
    }

    cg.vertices.get_mut(&course.key)
                .unwrap()
                .color = Color::Black;
    
    schedule.push(course.key.to_string());
}

fn find_topological_order<T>(
    course_num: usize,
    pre_requisites: Vec<Vec<T>>
)
where T: Eq + PartialEq + Clone + Hash + Display {
    let mut cg = build_graph(pre_requisites);
    let vertices = cg.vertices.clone();
    let mut courses = Vec::new();
    for key in vertices.keys() {
        courses.push(key);
    }

    let mut schedule = Vec::new();
    let has_circle = false;
    for i in 0..course_num {
        let course = cg.vertices.get(&courses[i])
                                                    .unwrap()
                                                    .clone();
        if !has_circle && Color::White == course.color {
            cg.vertices.get_mut(&courses[i])
                        .unwrap()
                        .color = Color::Gray;
            scheduling(&mut cg, course, &mut schedule, has_circle);
        }
    }

    if !has_circle {
        println!("{:#?}", schedule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course() {
        let course_num = 7;

        // 构 建 课 程 依 赖 关 系
        let mut pre_requisites = Vec::<Vec<&str>>::new();
        pre_requisites.push(vec!["微积分", "函数"]);
        pre_requisites.push(vec!["微积分", "导数"]);
        pre_requisites.push(vec!["线代", "方程组"]);
        pre_requisites.push(vec!["卷积网络", "微积分"]);
        pre_requisites.push(vec!["卷积网络", "概率论"]);
        pre_requisites.push(vec!["卷积网络", "线代"]);

        // 找 到 拓 扑 排 序 结 果 ， 即 合 理 的 课 程 学 习 顺 序
        find_topological_order(course_num, pre_requisites);
    }

    #[test]
    fn test_operation() {
        let operation_num = 9;

        // 构 建 做 菜 流 程 依 赖 关 系
        let mut pre_requisites = Vec::<Vec<&str>>::new();
        pre_requisites.push(vec!["混合", "3/4杯牛奶"]);
        pre_requisites.push(vec!["混合", "一个鸡蛋"]);
        pre_requisites.push(vec!["混合", " 一勺 橄 榄油 "]);
        pre_requisites.push(vec!["倒入1/4杯", "混合"]);
        pre_requisites.push(vec!["倒入1/4杯", "加热锅"]);
        pre_requisites.push(vec![" 底 面 金 黄 翻 面 ", "倒入1/4杯"]);
        pre_requisites.push(vec!["享用", " 底 面 金 黄 翻 面 "]);
        pre_requisites.push(vec!["享用", "加热糖浆"]);
        // 找 到 拓 扑 排 序 结 果 ， 即 合 理 的 做 菜 顺 序
        find_topological_order(operation_num, pre_requisites);
    }
}
