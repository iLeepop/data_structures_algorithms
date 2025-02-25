// deque.rs
// 双端队列

#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Deque {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    pub fn add_front(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }

        self.data.push(val);

        Ok(())
    }

    pub fn add_rear(&mut self, val: T) -> Result<(), String> {
        if Self::size(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);

        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            self.data.pop()
        } else {
            None
        }
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if Self::size(&self) > 0 {
            Some(self.data.remove(0))
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
    fn test_deque() {
        let mut d = Deque::new(4);
        let _r1 = d.add_front(1);
        let _r2 = d.add_front(2);
        let _r3 = d.add_rear(3);
        let _r4 = d.add_rear(4);
        if let Err(error) = d.add_front(5) {
            print!("add_front error: {error}")
        }

        if let Some(data) = d.remove_rear() {
            print!("remove_rear: {data}");
        } else {
            print!("empty queue");
        }

        println!("size: {}, is_empty: {}", d.size(), d.is_empty());
        println!("content: {:?}", d);
    }
}
