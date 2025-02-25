fn hash1(astr: &str, size: usize) -> usize {
    let mut sum = 0;
    for (i, c) in astr.chars().enumerate() {
        sum += (i + 1) * (c as usize);
    }
    sum % size
}

#[derive(Debug, Clone, PartialEq)]
struct HashMap<T> {
    size: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone + PartialEq + Default> HashMap<T> {
    fn new(size: usize) -> Self {
        let mut slot = Vec::with_capacity(size);
        let mut data = Vec::with_capacity(size);
        for _i in 0..size {
            slot.push(0);
            data.push(Default::default());
        }
        HashMap { size, slot, data }
    }

    fn hash(&self, key: usize) -> usize {
        key % self.size
    }

    fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.size
    }

    fn insert(&mut self, key: usize, value: T) {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                next = self.rehash(next);
                if next == pos {
                    println!("Error: slot is full, quit insertion");
                    return;
                }
            }

            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            None
        } else if key == self.slot[pos] {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if key == self.slot[curr] {
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }
            data
        }
    }

    fn get(&self, key: usize) -> Option<&T> {
        if 0 == key {
            panic!("Error: key must > 0");
        }

        let pos = self.hash(key);
        let mut data = None;
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        while 0 != self.slot[curr] && !found && !stop {
            if key == self.slot[curr] {
                found = true;
                data = self.data.get(curr);
            } else {
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        data
    }

    fn contains(&self, key: usize) -> bool {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        self.slot.contains(&key)
    }

    fn len(&self) -> usize {
        let mut length = 0;
        for d in self.slot.iter() {
            if &0 != d {
                length += 1;
            }
        }
        length
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash1() {
        let size = 11;
        let s1 = "rust";
        let s2 = "Rust";
        let p1 = hash1(s1, size);
        let p2 = hash1(s2, size);
        println!("{} {}", p1, p2);
    }

    #[test]
    fn test_hashmap() {
        let mut hm = HashMap::new(11);
        hm.insert(1, "Rust");
        hm.insert(3, "C");
        hm.insert(4, "Java");
        hm.insert(5, "Python");
        hm.insert(6, "JavaScript");

        println!("HashMap size {:?}", hm.len());
        println!("HashMap contains key 2: {}", hm.contains(2));
        println!("HashMap key 3: {:?}", hm.get(3));
        println!("HashMap remove key 3: {:?}", hm.remove(3));
        println!("HashMap remove key 3: {:?}", hm.remove(3));
    }
}
