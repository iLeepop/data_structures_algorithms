/// bfs

use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    data: usize,
    next: Link,
}

impl Node {
    fn new(data: usize) -> Self {
        Self {
            data: data,
            next: None
        }
    }
}

struct Graph {
    first: Link,
    last: Link,
}

impl Graph {
    fn new() -> Self {
        Self {
            first: None,
            last: None
        }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn get_first(&self) -> Link {
        self.first.clone()
    }

    fn print_node(&self) {
        let mut curr = self.first.clone();
        while let Some(val) = curr {
            print!("[{}]", &val.borrow().data);
            curr = val.borrow().next.clone();
        }

        print!("\n");
    }

    fn insert(&mut self, data: usize) {
        let node = Rc::new(RefCell::new(Node::new(data)));

        if self.is_empty() {
            self.first = Some(node.clone());
            self.last = Some(node);
        } else {
            self.last.as_mut()
                        .unwrap()
                        .borrow_mut()
                        .next = Some(node.clone());
            self.last = Some(node);
        }
    }
}

fn build_graph(data: [[usize;2];20]) -> Vec<(Graph, usize)> {
    let mut graphs: Vec<(Graph, usize)> = Vec::new();
    for _ in 0..9 { graphs.push((Graph::new(), 0)); }
    for i in 1..9 {
        for j in 0..data.len() {
            if data[j][0] == i {
                graphs[i].0.insert(data[j][1]);
            }
        }
        print!("[{i}]->");
        graphs[i].0.print_node();
    }
    graphs
}

fn bfs(graph: Vec<(Graph, usize)>) {
    let mut gp = graph;
    let mut nodes = Vec::new();
    gp[1].1 = 1;
    let mut curr = gp[1].0.get_first().clone();

    print!("{}->", 1);
    while let Some(val) = curr {
        nodes.push(val.borrow().data);
        curr = val.borrow().next.clone();
    }

    loop {
        if 0 == nodes.len() {
            break;
        } else {
            let data = nodes.remove(0);

            if 0 == gp[data].1 {
                gp[data].1 = 1;
                print!("{data}->");

                let mut curr = gp[data].0.get_first().clone();
                while let Some(val) = curr {
                    nodes.push(val.borrow().data);
                    curr = val.borrow().next.clone();
                }
            }
        }
    }
    println!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let data = [
            [1,2],[2,1],[1,3],[3,1],[2,4],[4,2],[2,5],
            [5,2],[3,6],[6,3],[3,7],[7,3],[4,5],[5,4],
            [6,7],[7,6],[5,8],[8,5],[6,8],[8,6]
        ];
        let gp = build_graph(data);
        bfs(gp);
    }
        
}

