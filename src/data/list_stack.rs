// list_stack.rs
// 链表栈

#[derive(Debug, Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {data: data, next: None}
    }
}

#[derive(Debug, Clone)]
pub struct Stack<T> {
    size: usize,
    top: Link<T>,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { size: 0, top: None }
    }

    pub fn push(&mut self, val: T) {
        let mut node = Node::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_stack() {
        let mut s = Stack::new();
        s.push(1); s.push(2); s.push(3);
        println!("top {:?}, size {}", s.peek(), s.size());
        println!("pop {:?}, size {}", s.pop(), s.size());
        println!("is_empty {}, stack: {:?}", s.is_empty(), s);
    }
}