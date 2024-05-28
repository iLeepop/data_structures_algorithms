// stack.rs

#[derive(Debug)]
pub struct Stack<T> {
    top: usize,   // 栈顶
    data: Vec<T>, // 栈数据容器
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val); // 数据压入栈 放在队尾
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 { return None; }
        self.top -= 1;
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 { return None; }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        0 == self.top
    }

    pub fn size(&self) -> usize {
        self.top
    }
}