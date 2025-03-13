/// avl

use std::cmp::Ordering::*;
use std::cmp::max;
use std::mem::replace;
use AvlTree::*;

#[derive(Debug)]
enum AvlTree<T> {
    Null,
    Tree(Box<AvlNode<T>>),
}

#[derive(Debug)]
struct AvlNode<T> {
    val: T,
    left: AvlTree<T>,
    right: AvlTree<T>,
    bfactor: i8,
}

impl<T> AvlTree<T>
where T: Ord
{
    fn new() -> AvlTree<T> {
        Null
    }

    fn insert(&mut self, val: T) -> (bool, bool) {
        let ret = match *self {
            Null => {
                let node = AvlNode {
                    val: val,
                    left: Null,
                    right: Null,
                    bfactor: 0,
                };
                *self = Tree(Box::new(node));
                (true, true)
            },
            Tree(ref mut node) => match node.val.cmp(&val) {
                Equal => (false, false),
                Less => {
                    let (inserted, deepened) = node.right.insert(val);
                    if deepened {
                        let ret = match node.bfactor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.bfactor += 1;
                        ret
                    } else {
                        (inserted, deepened)
                    }
                },
                Greater => {
                    let (inserted, deepened) = node.left.insert(val);
                    if deepened {
                        let ret = match node.bfactor {
                            -1 => (inserted, false),
                            0 => (inserted, true),
                            1 => (inserted, false),
                            _ => unreachable!(),
                        };
                        node.bfactor -= 1;
                        ret
                    } else {
                        (inserted, deepened)
                    }
                }
            }
        };
        self.rebalance();
        ret
    }

    fn rebalance(&mut self) {
        match *self {
            Null => (),
            Tree(_) => match self.node().bfactor {
                -2 => {
                    let lbf = self.node().left.node().bfactor;
                    if lbf == -1 || lbf == 0 {
                        let (a, b) = if lbf == -1 {
                            (0, 0)
                        } else {
                            (-1, 1)
                        };
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if lbf == 1 {
                        let (a, b) = match self.node().left.node().right.node().bfactor {
                            -1 => (1, 0),
                            0 => (0, 0),
                            1 => (0, -1),
                            _ => unreachable!(),
                        };
                        self.node().left.rotate_left();
                        self.rotate_right();
                        self.node().right.node().bfactor = a;
                        self.node().left.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                },
                2 => {
                    let rbf = self.node().right.node().bfactor;
                    if rbf == 1 || rbf == 0 {
                        let (a, b) = if rbf == 1 {
                            (0, 0)
                        } else {
                            (1, -1)
                        };
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().bfactor = b;
                    } else if rbf == -1 {
                        let (a, b) = match self.node().right.node().left.node().bfactor {
                            1 => (-1, 0),
                            0 => (0, 0),
                            -1 => (0, 1),
                            _ => unreachable!(),
                        };

                        self.node().right.rotate_right();
                        self.rotate_left();
                        self.node().left.node().bfactor = a;
                        self.node().right.node().bfactor = b;
                        self.node().bfactor = 0;
                    } else {
                        unreachable!()
                    }
                },
                _ => (),
            },
        }
    }

    fn node(&mut self) -> &mut AvlNode<T> {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut n) => n,
        }
    }

    fn left_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.left,
        }
    }

    fn right_subtree(&mut self) -> &mut Self {
        match *self {
            Null => panic!("Empty tree"),
            Tree(ref mut node) => &mut node.right,
        }
    }

    fn rotate_left(&mut self) {
        let mut v = replace(self, Null);
        let mut right = replace(v.right_subtree(), Null);
        let right_left = replace(right.left_subtree(), Null);
        *v.right_subtree() = right_left;
        *right.left_subtree() = v;
        *self = right;
    }

    fn rotate_right(&mut self) {
        let mut v = replace(self, Null);
        let mut left = replace(v.left_subtree(), Null);
        let left_right = replace(left.right_subtree(), Null);
        *v.left_subtree() = left_right;
        *left.right_subtree() = v;
        *self = left;
    }

}

impl<T> AvlTree<T>
where T: Ord
{
    fn len(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref v) => 1 + v.left.len() + v.right.len(),
        }
    }

    fn depth(&self) -> usize {
        match *self {
            Null => 0,
            Tree(ref v) => max(v.left.depth(), v.right.depth()) + 1,
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            Null => true,
            _ => false,
        }
    }

    fn search(&self, val: &T) -> bool {
        match *self {
            Null => false,
            Tree(ref v) => {
                match v.val.cmp(&val) {
                    Equal => { true },
                    Greater => {
                        match &v.left {
                            Null => false,
                            _ => v.left.search(val),
                        }
                    },
                    Less => {
                        match &v.right {
                            Null => false,
                            _ => v.right.search(val),
                        }
                    }
                }
            }
        }
    }
}