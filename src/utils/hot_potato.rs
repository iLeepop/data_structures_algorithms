// hot_potato.rs

use crate::Queue;

pub fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(names.len());
    for name in names {
        let _rm = q.enqueue(name);
    }

    while q.size() > 1 {
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }

        let _rm = q.dequeue();
    }

    q.dequeue().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hot_potato() {
        let name = vec!["Bill", "David", "Susan", "Jane", "Kew", "Brad"];
        let rem = hot_potato(name, 2);
        println!("The left person is {rem}");
    }
}