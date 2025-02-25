// linked_list.rs
// 链表

pub type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    size: usize,
    head: Link<T>,
}

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            size: 0,
            head: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // into_iter: 链表改变 成为迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // iter: 链表不变 只得到不可变迭代器
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    // iter_mut: 链表不变 得到可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        fn base() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.peek(), Some(&2));
            assert_eq!(list.peek_mut(), Some(&mut 2));
            list.peek_mut().map(|val| {
                *val = 4;
            });
            assert_eq!(list.peek(), Some(&4));
        }

        fn into_iter() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);
            let mut iter = list.into_iter();
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), None);
        }

        fn iter() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);
            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), None);
        }

        fn iter_mut() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);
            let mut iter = list.iter_mut();
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), None);
        }
        base();
        into_iter();
        iter();
        iter_mut();
    }
}
