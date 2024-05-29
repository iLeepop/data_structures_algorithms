// queue.rs
// 队列

#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Queue {
            data: Vec::with_capacity(size),
            cap: size,
        }
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("Noe space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == Self::size(&self)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new(3);
        let _r1 = q.enqueue(1);
        let _r2 = q.enqueue(2);
        let _r3 = q.enqueue(3);
        if let Err(error) = q.enqueue(4) {
            println!("{}", error);
        }

        if let Some(data) = q.dequeue() {
            println!("{}", data);
        } else {
            println!("empty queue");
        }

        println!("size: {}, empty: {}", q.size(), q.is_empty());
        println!("content: {:?}", q);
    }
}